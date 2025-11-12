/// Praeda C# Wrapper
///
/// High-level C# interface to the Praeda Rust loot generation library
/// Uses System.Text.Json for JSON handling

using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Praeda {
    /// <summary>
    /// Main Praeda generator class for C#
    ///
    /// Usage example:
    /// <code>
    /// using var gen = new PraedaGenerator();
    ///
    /// // Load configuration
    /// gen.LoadTomlString(@"
    /// [quality_data]
    /// common = 100
    /// uncommon = 60
    /// rare = 30
    /// ");
    ///
    /// // Generate items
    /// var options = new {
    ///     number_of_items = 10,
    ///     base_level = 15.0,
    ///     affix_chance = 0.75,
    ///     linear = true,
    ///     scaling_factor = 1.0
    /// };
    ///
    /// var optionsJson = JsonSerializer.Serialize(options);
    /// var itemsJson = gen.GenerateLoot(optionsJson);
    ///
    /// Console.WriteLine(itemsJson);
    /// </code>
    /// </summary>
    public class PraedaGenerator : IDisposable {
        private IntPtr handle;
        private bool disposed = false;

        /// <summary>
        /// Import declarations for C FFI functions
        /// </summary>
        private static class NativeMethods {
            private const string DLL_NAME = "praeda"; // or "praeda.dll" on Windows

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_generator_new();

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern void praeda_generator_free(IntPtr handle);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern void praeda_string_free(IntPtr ptr);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_generator_load_toml(IntPtr handle, string toml_str);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_generator_generate_loot(IntPtr handle, string options_json);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_generator_get_quality_data(IntPtr handle);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern int praeda_generator_has_quality(IntPtr handle, string quality);

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_version();

            [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr praeda_generator_info(IntPtr handle);
        }

        /// <summary>
        /// Create a new Praeda generator
        /// </summary>
        public PraedaGenerator() {
            handle = NativeMethods.praeda_generator_new();
            if (handle == IntPtr.Zero) {
                throw new InvalidOperationException("Failed to create Praeda generator");
            }
        }

        /// <summary>
        /// Destructor
        /// </summary>
        ~PraedaGenerator() {
            Dispose(false);
        }

        /// <summary>
        /// Load configuration from a TOML string
        /// </summary>
        /// <param name="tomlContent">TOML configuration content</param>
        /// <exception cref="InvalidOperationException">If loading fails</exception>
        public void LoadTomlString(string tomlContent) {
            ThrowIfDisposed();
            IntPtr resultPtr = NativeMethods.praeda_generator_load_toml(handle, tomlContent);
            string result = MarshalString(resultPtr);
            CheckResult(result);
        }

        /// <summary>
        /// Load configuration from a TOML file
        /// </summary>
        /// <param name="filePath">Path to TOML configuration file</param>
        /// <exception cref="System.IO.FileNotFoundException">If file not found</exception>
        /// <exception cref="InvalidOperationException">If loading fails</exception>
        public void LoadTomlFile(string filePath) {
            string content = System.IO.File.ReadAllText(filePath);
            LoadTomlString(content);
        }

        /// <summary>
        /// Generate loot items
        /// </summary>
        /// <param name="optionsJson">Generation options as JSON string</param>
        /// <returns>JSON array of generated items</returns>
        /// <exception cref="InvalidOperationException">If generation fails</exception>
        ///
        /// <remarks>
        /// Options JSON should contain:
        /// {
        ///   "number_of_items": 10,
        ///   "base_level": 15.0,
        ///   "level_variance": 5.0,
        ///   "affix_chance": 0.75,
        ///   "linear": true,
        ///   "scaling_factor": 1.0
        /// }
        /// </remarks>
        public string GenerateLoot(string optionsJson) {
            ThrowIfDisposed();
            IntPtr resultPtr = NativeMethods.praeda_generator_generate_loot(handle, optionsJson);
            string result = MarshalString(resultPtr);
            if (string.IsNullOrEmpty(result)) {
                throw new InvalidOperationException("Failed to generate loot");
            }
            return result;
        }

        /// <summary>
        /// Get quality data
        /// </summary>
        /// <returns>JSON object mapping quality names to weights</returns>
        public string GetQualityData() {
            ThrowIfDisposed();
            IntPtr resultPtr = NativeMethods.praeda_generator_get_quality_data(handle);
            return MarshalString(resultPtr);
        }

        /// <summary>
        /// Check if a quality exists
        /// </summary>
        /// <param name="quality">Quality name</param>
        /// <returns>True if quality exists, false otherwise</returns>
        /// <exception cref="InvalidOperationException">On error</exception>
        public bool HasQuality(string quality) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_has_quality(handle, quality);
            if (result < 0) {
                throw new InvalidOperationException("Error checking quality");
            }
            return result == 1;
        }

        /// <summary>
        /// Get generator information
        /// </summary>
        /// <returns>JSON object with generator info</returns>
        public string GetInfo() {
            ThrowIfDisposed();
            IntPtr resultPtr = NativeMethods.praeda_generator_info(handle);
            return MarshalString(resultPtr);
        }

        /// <summary>
        /// Get the Praeda library version
        /// </summary>
        public static string GetVersion() {
            IntPtr versionPtr = NativeMethods.praeda_version();
            return MarshalString(versionPtr);
        }

        /// <summary>
        /// Dispose the generator
        /// </summary>
        public void Dispose() {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        /// <summary>
        /// Protected dispose method
        /// </summary>
        protected virtual void Dispose(bool disposing) {
            if (!disposed) {
                if (handle != IntPtr.Zero) {
                    NativeMethods.praeda_generator_free(handle);
                    handle = IntPtr.Zero;
                }
                disposed = true;
            }
        }

        /// <summary>
        /// Convert IntPtr to string (marshaling from C)
        /// </summary>
        private static string MarshalString(IntPtr ptr) {
            if (ptr == IntPtr.Zero) return string.Empty;
            string result = Marshal.PtrToStringAnsi(ptr) ?? string.Empty;
            NativeMethods.praeda_string_free(ptr);
            return result;
        }

        /// <summary>
        /// Check result JSON for errors
        /// </summary>
        private void CheckResult(string json) {
            if (string.IsNullOrEmpty(json)) {
                throw new InvalidOperationException("Empty response from Praeda");
            }

            try {
                using JsonDocument doc = JsonDocument.Parse(json);
                var root = doc.RootElement;

                // Check for success field
                if (root.TryGetProperty("success", out var successElement)) {
                    if (successElement.ValueKind == JsonValueKind.False) {
                        string errorMsg = "Operation failed";
                        if (root.TryGetProperty("error", out var errorElement)) {
                            errorMsg = errorElement.GetString() ?? errorMsg;
                        }
                        throw new InvalidOperationException(errorMsg);
                    }
                }

                // Check for error field
                if (root.TryGetProperty("error", out var errorField)) {
                    throw new InvalidOperationException(errorField.GetString() ?? "Unknown error");
                }
            } catch (JsonException ex) {
                throw new InvalidOperationException($"Invalid JSON response: {ex.Message}");
            }
        }

        /// <summary>
        /// Check if disposed
        /// </summary>
        private void ThrowIfDisposed() {
            if (disposed) {
                throw new ObjectDisposedException(nameof(PraedaGenerator));
            }
        }
    }

    /// <summary>
    /// Generation options for loot generation
    /// </summary>
    [Serializable]
    public class GenerationOptions {
        [JsonPropertyName("number_of_items")]
        public uint NumberOfItems { get; set; } = 1;

        [JsonPropertyName("base_level")]
        public double BaseLevel { get; set; } = 10.0;

        [JsonPropertyName("level_variance")]
        public double LevelVariance { get; set; } = 5.0;

        [JsonPropertyName("affix_chance")]
        public double AffixChance { get; set; } = 0.75;

        [JsonPropertyName("linear")]
        public bool Linear { get; set; } = true;

        [JsonPropertyName("scaling_factor")]
        public double ScalingFactor { get; set; } = 1.0;
    }

    /// <summary>
    /// Helper class for working with Praeda JSON results
    /// </summary>
    public static class PraedaHelper {
        /// <summary>
        /// Generate loot with strongly-typed options
        /// </summary>
        public static string GenerateLoot(PraedaGenerator gen, GenerationOptions options) {
            var json = JsonSerializer.Serialize(options);
            return gen.GenerateLoot(json);
        }

        /// <summary>
        /// Parse loot JSON response (if you have Item type defined)
        /// </summary>
        public static JsonElement ParseLootResponse(string json) {
            try {
                using var doc = JsonDocument.Parse(json);
                return doc.RootElement.Clone();
            } catch (JsonException ex) {
                throw new InvalidOperationException($"Failed to parse loot response: {ex.Message}");
            }
        }

        /// <summary>
        /// Get formatted quality data as a dictionary
        /// </summary>
        public static System.Collections.Generic.Dictionary<string, int> GetQualityDataDictionary(string json) {
            try {
                return JsonSerializer.Deserialize<System.Collections.Generic.Dictionary<string, int>>(json) ??
                    new System.Collections.Generic.Dictionary<string, int>();
            } catch (JsonException ex) {
                throw new InvalidOperationException($"Failed to parse quality data: {ex.Message}");
            }
        }
    }
}
