/// Praeda C# Wrapper
///
/// High-level C# interface to the Praeda Rust loot generation library
/// Zero JSON - all data exchanged through native C# types

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Praeda {
    // ============================================================================
    // C Interop Declarations
    // ============================================================================

    /// <summary>
    /// C-compatible ItemAttribute struct
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct CItemAttribute {
        public IntPtr Name;
        public double InitialValue;
        public double Min;
        public double Max;
        public byte Required;
        public double ScalingFactor;
        public double Chance;
    }

    /// <summary>
    /// C-compatible Affix struct
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct CAffix {
        public IntPtr Name;
        public IntPtr Attributes;
        public uint AttributesCount;
    }

    /// <summary>
    /// C-compatible Item struct
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct CItem {
        public IntPtr Name;
        public IntPtr Quality;
        public IntPtr ItemType;
        public IntPtr Subtype;
        public CAffix Prefix;
        public CAffix Suffix;
        public IntPtr Attributes;
        public uint AttributesCount;
    }

    /// <summary>
    /// Native methods from Rust FFI
    /// </summary>
    internal static class NativeMethods {
        private const string DllName = "praeda";

        [DllImport(DllName)]
        public static extern IntPtr praeda_generator_new();

        [DllImport(DllName)]
        public static extern void praeda_generator_free(IntPtr handle);

        [DllImport(DllName)]
        public static extern void praeda_string_free(IntPtr ptr);

        [DllImport(DllName)]
        public static extern void praeda_error_free(IntPtr ptr);

        [DllImport(DllName)]
        public static extern void praeda_item_array_free(IntPtr handle);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_load_toml(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string tomlStr,
            out IntPtr errorOut
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_set_quality_data(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string qualityName,
            int weight
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_set_item_type(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string typeName,
            int weight
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_set_item_subtype(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string typeName,
            [MarshalAs(UnmanagedType.LPStr)] string subtypeName,
            int weight
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_set_attribute(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string typeName,
            [MarshalAs(UnmanagedType.LPStr)] string subtypeName,
            [MarshalAs(UnmanagedType.LPStr)] string attrName,
            double initialValue,
            double minValue,
            double maxValue,
            int required
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_set_item_names(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPStr)] string typeName,
            [MarshalAs(UnmanagedType.LPStr)] string subtypeName,
            [MarshalAs(UnmanagedType.LPArray, ArraySubType = UnmanagedType.LPStr)] string[] names,
            uint namesCount
        );

        [DllImport(DllName)]
        public static extern IntPtr praeda_generator_generate_loot(
            IntPtr handle,
            uint numberOfItems,
            double baseLevel,
            double levelVariance,
            double affixChance,
            byte linear,
            double scalingFactor,
            out IntPtr errorOut
        );

        [DllImport(DllName)]
        public static extern uint praeda_item_array_count(IntPtr handle);

        [DllImport(DllName)]
        public static extern IntPtr praeda_item_array_get(IntPtr handle, uint index);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int praeda_generator_has_quality(IntPtr handle, [MarshalAs(UnmanagedType.LPStr)] string quality);

        [DllImport(DllName)]
        public static extern IntPtr praeda_version();
    }

    // ============================================================================
    // C# Native Classes
    // ============================================================================

    /// <summary>
    /// Item attribute - mirrors Rust ItemAttribute struct
    /// </summary>
    public class ItemAttribute {
        public string Name { get; set; } = "";
        public double InitialValue { get; set; }
        public double Min { get; set; }
        public double Max { get; set; }
        public bool Required { get; set; }
        public double ScalingFactor { get; set; } = 1.0;
        public double Chance { get; set; }

        public ItemAttribute() { }
        public ItemAttribute(string name, double initialValue, double min, double max, bool required) {
            Name = name;
            InitialValue = initialValue;
            Min = min;
            Max = max;
            Required = required;
            ScalingFactor = 1.0;
            Chance = 0.0;
        }

        internal static ItemAttribute FromNative(CItemAttribute cAttr) {
            return new ItemAttribute {
                Name = Marshal.PtrToStringUTF8(cAttr.Name) ?? "",
                InitialValue = cAttr.InitialValue,
                Min = cAttr.Min,
                Max = cAttr.Max,
                Required = cAttr.Required != 0,
                ScalingFactor = cAttr.ScalingFactor,
                Chance = cAttr.Chance
            };
        }
    }

    /// <summary>
    /// Affix (prefix or suffix) - mirrors Rust Affix struct
    /// </summary>
    public class Affix {
        public string Name { get; set; } = "";
        public List<ItemAttribute> Attributes { get; set; } = new();

        public Affix() { }
        public Affix(string name, List<ItemAttribute> attributes) {
            Name = name;
            Attributes = attributes;
        }

        internal static Affix FromNative(CAffix cAffix) {
            var affix = new Affix {
                Name = Marshal.PtrToStringUTF8(cAffix.Name) ?? ""
            };

            if (cAffix.Attributes != IntPtr.Zero && cAffix.AttributesCount > 0) {
                int structSize = Marshal.SizeOf<CItemAttribute>();
                for (uint i = 0; i < cAffix.AttributesCount; i++) {
                    IntPtr attrPtr = IntPtr.Add(cAffix.Attributes, (int)(i * structSize));
                    CItemAttribute cAttr = Marshal.PtrToStructure<CItemAttribute>(attrPtr);
                    affix.Attributes.Add(ItemAttribute.FromNative(cAttr));
                }
            }

            return affix;
        }
    }

    /// <summary>
    /// Generated item - mirrors Rust Item struct
    /// </summary>
    public class Item {
        public string Name { get; set; } = "";
        public string Quality { get; set; } = "";
        public string Type { get; set; } = "";
        public string Subtype { get; set; } = "";
        public Affix Prefix { get; set; } = new();
        public Affix Suffix { get; set; } = new();
        public Dictionary<string, ItemAttribute> Attributes { get; set; } = new();

        public Item() { }

        internal static Item FromNative(CItem cItem) {
            var item = new Item {
                Name = Marshal.PtrToStringUTF8(cItem.Name) ?? "",
                Quality = Marshal.PtrToStringUTF8(cItem.Quality) ?? "",
                Type = Marshal.PtrToStringUTF8(cItem.ItemType) ?? "",
                Subtype = Marshal.PtrToStringUTF8(cItem.Subtype) ?? "",
                Prefix = Affix.FromNative(cItem.Prefix),
                Suffix = Affix.FromNative(cItem.Suffix)
            };

            if (cItem.Attributes != IntPtr.Zero && cItem.AttributesCount > 0) {
                int structSize = Marshal.SizeOf<CItemAttribute>();
                for (uint i = 0; i < cItem.AttributesCount; i++) {
                    IntPtr attrPtr = IntPtr.Add(cItem.Attributes, (int)(i * structSize));
                    CItemAttribute cAttr = Marshal.PtrToStructure<CItemAttribute>(attrPtr);
                    ItemAttribute attr = ItemAttribute.FromNative(cAttr);
                    item.Attributes[attr.Name] = attr;
                }
            }

            return item;
        }
    }

    /// <summary>
    /// Generation options - mirrors Rust GeneratorOptions struct
    /// </summary>
    public class GenerationOptions {
        public uint NumberOfItems { get; set; } = 1;
        public double BaseLevel { get; set; } = 1.0;
        public double LevelVariance { get; set; } = 1.0;
        public double AffixChance { get; set; } = 0.25;
        public bool Linear { get; set; } = true;
        public double ScalingFactor { get; set; } = 1.0;
    }

    /// <summary>
    /// Main Praeda generator class
    /// </summary>
    public class PraedaGenerator : IDisposable {
        private IntPtr handle;
        private bool disposed = false;

        public PraedaGenerator() {
            handle = NativeMethods.praeda_generator_new();
            if (handle == IntPtr.Zero) {
                throw new InvalidOperationException("Failed to create generator");
            }
        }

        public void Dispose() {
            if (!disposed) {
                if (handle != IntPtr.Zero) {
                    NativeMethods.praeda_generator_free(handle);
                    handle = IntPtr.Zero;
                }
                disposed = true;
            }
            GC.SuppressFinalize(this);
        }

        ~PraedaGenerator() {
            Dispose();
        }

        private void ThrowIfDisposed() {
            if (disposed) {
                throw new ObjectDisposedException("PraedaGenerator");
            }
        }

        private static string MarshalString(IntPtr ptr) {
            if (ptr == IntPtr.Zero) {
                return "";
            }
            string result = Marshal.PtrToStringUTF8(ptr) ?? "";
            NativeMethods.praeda_string_free(ptr);
            return result;
        }

        /// <summary>
        /// Load configuration from TOML string
        /// </summary>
        public void LoadTomlString(string tomlContent) {
            ThrowIfDisposed();
            IntPtr errorPtr = IntPtr.Zero;
            int result = NativeMethods.praeda_generator_load_toml(handle, tomlContent, out errorPtr);

            if (result != 0) {
                string errorMsg = MarshalString(errorPtr);
                throw new InvalidOperationException($"Failed to load TOML: {errorMsg}");
            }
        }

        /// <summary>
        /// Set quality tier data
        /// </summary>
        public bool SetQualityData(string qualityName, int weight) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_set_quality_data(handle, qualityName, weight);
            return result == 0;
        }

        /// <summary>
        /// Set item type with weight
        /// </summary>
        public bool SetItemType(string typeName, int weight) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_set_item_type(handle, typeName, weight);
            return result == 0;
        }

        /// <summary>
        /// Set item subtype with weight
        /// </summary>
        public bool SetItemSubtype(string typeName, string subtypeName, int weight) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_set_item_subtype(handle, typeName, subtypeName, weight);
            return result == 0;
        }

        /// <summary>
        /// Set attribute for an item type/subtype
        /// </summary>
        public bool SetAttribute(string typeName, string subtypeName, string attrName,
                                double initialValue, double minValue, double maxValue, bool required) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_set_attribute(
                handle, typeName, subtypeName, attrName,
                initialValue, minValue, maxValue, required ? 1 : 0
            );
            return result == 0;
        }

        /// <summary>
        /// Set item names for a type/subtype combination
        /// </summary>
        public bool SetItemNames(string typeName, string subtypeName, string[] names) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_set_item_names(
                handle, typeName, subtypeName, names, (uint)names.Length
            );
            return result == 0;
        }

        /// <summary>
        /// Generate loot items
        /// </summary>
        public List<Item> GenerateLoot(GenerationOptions options) {
            ThrowIfDisposed();

            IntPtr errorPtr = IntPtr.Zero;
            IntPtr arrayHandle = NativeMethods.praeda_generator_generate_loot(
                handle,
                options.NumberOfItems,
                options.BaseLevel,
                options.LevelVariance,
                options.AffixChance,
                (byte)(options.Linear ? 1 : 0),
                options.ScalingFactor,
                out errorPtr
            );

            if (arrayHandle == IntPtr.Zero) {
                string errorMsg = MarshalString(errorPtr);
                throw new InvalidOperationException($"Failed to generate loot: {errorMsg}");
            }

            var items = new List<Item>();
            uint count = NativeMethods.praeda_item_array_count(arrayHandle);

            for (uint i = 0; i < count; i++) {
                IntPtr itemPtr = NativeMethods.praeda_item_array_get(arrayHandle, i);
                if (itemPtr != IntPtr.Zero) {
                    CItem cItem = Marshal.PtrToStructure<CItem>(itemPtr);
                    items.Add(Item.FromNative(cItem));
                }
            }

            NativeMethods.praeda_item_array_free(arrayHandle);
            return items;
        }

        /// <summary>
        /// Check if a quality exists
        /// </summary>
        public bool HasQuality(string quality) {
            ThrowIfDisposed();
            int result = NativeMethods.praeda_generator_has_quality(handle, quality);
            if (result < 0) {
                throw new InvalidOperationException("Error checking quality");
            }
            return result == 1;
        }

        /// <summary>
        /// Get generator info
        /// </summary>
        public string GetInfo() {
            ThrowIfDisposed();
            IntPtr versionPtr = NativeMethods.praeda_version();
            return MarshalString(versionPtr);
        }
    }
}
