#!/usr/bin/env bash
set -euo pipefail

# ── MIDI Note to Program Change — Cross-Platform Build Script ──────────────
# Usage:
#   ./build.sh              # Build all platforms
#   ./build.sh windows      # Build Windows only
#   ./build.sh macos        # Build macOS only (Universal2: Intel + Apple Silicon)
#   ./build.sh linux        # Build Linux only

MACOS_SDK="/opt/macos-sdk/MacOSX14.5.sdk"
PLUGIN_NAME="midi_note_to_pc"
OUTPUT_DIR="."

mkdir -p "$OUTPUT_DIR"

build_linux() {
    echo "━━━ Building for Linux x86_64 ━━━"
    cargo xtask bundle "$PLUGIN_NAME" --release
    
    # Copy to output
    cp -r "target/bundled/${PLUGIN_NAME}.vst3" "$OUTPUT_DIR/${PLUGIN_NAME}-linux-x86_64.vst3"
    cp "target/bundled/${PLUGIN_NAME}.clap" "$OUTPUT_DIR/${PLUGIN_NAME}-linux-x86_64.clap"
    
    echo "✅ Linux build: $OUTPUT_DIR/${PLUGIN_NAME}-linux-x86_64.{vst3,clap}"
}

build_windows() {
    echo "━━━ Building for Windows x86_64 ━━━"
    cargo xtask bundle "$PLUGIN_NAME" --release --target x86_64-pc-windows-gnu
    
    # Copy to output
    cp -r "target/bundled/${PLUGIN_NAME}.vst3" "$OUTPUT_DIR/${PLUGIN_NAME}-windows-x86_64.vst3"
    cp "target/bundled/${PLUGIN_NAME}.clap" "$OUTPUT_DIR/${PLUGIN_NAME}-windows-x86_64.clap"
    
    echo "✅ Windows build: $OUTPUT_DIR/${PLUGIN_NAME}-windows-x86_64.{vst3,clap}"
}

build_macos() {
    echo "━━━ Building for macOS Universal2 (Intel + Apple Silicon) ━━━"

    if [ ! -d "$MACOS_SDK" ]; then
        echo "❌ macOS SDK not found at $MACOS_SDK"
        echo "   Download it with:"
        echo '   curl -L "https://github.com/joseluisq/macosx-sdks/releases/download/14.5/MacOSX14.5.sdk.tar.xz" -o /tmp/MacOSX14.5.sdk.tar.xz'
        echo '   sudo mkdir -p /opt/macos-sdk && sudo tar -xJf /tmp/MacOSX14.5.sdk.tar.xz -C /opt/macos-sdk/'
        exit 1
    fi

    # Build both architectures with zigbuild
    echo "  → Building x86_64-apple-darwin..."
    SDKROOT="$MACOS_SDK" COREAUDIO_SDK_PATH="$MACOS_SDK" \
        cargo zigbuild --release --target x86_64-apple-darwin

    echo "  → Building aarch64-apple-darwin..."
    SDKROOT="$MACOS_SDK" COREAUDIO_SDK_PATH="$MACOS_SDK" \
        cargo zigbuild --release --target aarch64-apple-darwin

    # Create the Universal2 VST3 bundle manually
    echo "  → Creating Universal2 bundles..."
    
    # VST3 bundle
    local vst3_dir="$OUTPUT_DIR/${PLUGIN_NAME}-macos-universal.vst3/Contents/MacOS"
    mkdir -p "$vst3_dir"
    
    # Use zig's lipo equivalent or system lipo if available
    if command -v lipo &> /dev/null; then
        lipo -create \
            "target/x86_64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            "target/aarch64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            -output "$vst3_dir/$PLUGIN_NAME"
    else
        # Use zig's objcopy to create a fat binary
        # Or just provide both architectures separately
        echo "  ⚠ lipo not available — creating separate architecture bundles"
        
        # x86_64 VST3
        local vst3_x64="$OUTPUT_DIR/${PLUGIN_NAME}-macos-x86_64.vst3/Contents/MacOS"
        mkdir -p "$vst3_x64"
        cp "target/x86_64-apple-darwin/release/libmidi_note_to_pc.dylib" "$vst3_x64/$PLUGIN_NAME"
        
        # Create Info.plist for x86_64
        create_vst3_plist "$OUTPUT_DIR/${PLUGIN_NAME}-macos-x86_64.vst3/Contents/Info.plist"
        
        # aarch64 VST3
        local vst3_arm="$OUTPUT_DIR/${PLUGIN_NAME}-macos-aarch64.vst3/Contents/MacOS"
        mkdir -p "$vst3_arm"
        cp "target/aarch64-apple-darwin/release/libmidi_note_to_pc.dylib" "$vst3_arm/$PLUGIN_NAME"
        
        # Create Info.plist for aarch64
        create_vst3_plist "$OUTPUT_DIR/${PLUGIN_NAME}-macos-aarch64.vst3/Contents/Info.plist"
        
        # CLAP bundles (separate architectures)
        cp "target/x86_64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            "$OUTPUT_DIR/${PLUGIN_NAME}-macos-x86_64.clap"
        cp "target/aarch64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            "$OUTPUT_DIR/${PLUGIN_NAME}-macos-aarch64.clap"
        
        echo "✅ macOS builds (separate archs):"
        echo "   $OUTPUT_DIR/${PLUGIN_NAME}-macos-x86_64.{vst3,clap}"
        echo "   $OUTPUT_DIR/${PLUGIN_NAME}-macos-aarch64.{vst3,clap}"
        return
    fi
    
    # Create Info.plist
    create_vst3_plist "$OUTPUT_DIR/${PLUGIN_NAME}-macos-universal.vst3/Contents/Info.plist"
    
    # CLAP bundle (Universal2)
    if command -v lipo &> /dev/null; then
        lipo -create \
            "target/x86_64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            "target/aarch64-apple-darwin/release/libmidi_note_to_pc.dylib" \
            -output "$OUTPUT_DIR/${PLUGIN_NAME}-macos-universal.clap"
    fi
    
    echo "✅ macOS Universal2: $OUTPUT_DIR/${PLUGIN_NAME}-macos-universal.{vst3,clap}"
}

create_vst3_plist() {
    cat > "$1" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>midi_note_to_pc</string>
    <key>CFBundleIconFile</key>
    <string></string>
    <key>CFBundleIdentifier</key>
    <string>com.nico.midi-note-to-pc</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>MIDI Note to Program Change</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
</dict>
</plist>
PLIST
}

# ── Main ───────────────────────────────────────────────────────────────────
case "${1:-all}" in
    linux)   build_linux ;;
    windows) build_windows ;;
    macos)   build_macos ;;
    all)
        build_linux
        echo ""
        build_windows
        echo ""
        build_macos
        echo ""
        echo "━━━ All builds complete! ━━━"
        echo "Output directory: $OUTPUT_DIR/"
        ls -lhR "$OUTPUT_DIR/"
        ;;
    *)
        echo "Usage: $0 [linux|windows|macos|all]"
        exit 1
        ;;
esac
