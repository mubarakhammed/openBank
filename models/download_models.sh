#!/bin/bash

# OpenBank Identity Models Download Script
# This script downloads placeholder/demo models for development

set -e

echo "🤖 OpenBank Identity Models Setup"
echo "=================================="

MODEL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo "📁 Models directory: $MODEL_DIR"

# Create models directory if it doesn't exist
mkdir -p "$MODEL_DIR"

# Function to download a file if it doesn't exist
download_if_missing() {
    local url=$1
    local filename=$2
    local description=$3
    
    if [ ! -f "$MODEL_DIR/$filename" ]; then
        echo "⬇️  Downloading $description..."
        echo "   URL: $url"
        echo "   File: $filename"
        # curl -L -o "$MODEL_DIR/$filename" "$url"
        echo "   ⚠️  Manual download required - automated download disabled for security"
        echo "   Please download manually and place in models/ directory"
    else
        echo "✅ $filename already exists"
    fi
}

echo ""
echo "📋 Required Models:"
echo "==================="

# Face Detection Model (YOLOv5-face)
echo "1. Face Detection Model"
download_if_missing \
    "https://github.com/deepcam-cn/yolov5-face/releases/download/v0.0.0/yolov5n-face.onnx" \
    "face_detection.onnx" \
    "YOLOv5-Face Detection Model"

# Face Recognition Model (InsightFace)
echo "2. Face Recognition Model" 
download_if_missing \
    "https://github.com/onnx/models/raw/main/vision/body_analysis/arcface/model/arcfaceresnet100-8.onnx" \
    "face_recognition.onnx" \
    "ArcFace Recognition Model"

# Liveness Detection Model
echo "3. Liveness Detection Model"
download_if_missing \
    "https://example.com/liveness-model.onnx" \
    "liveness.onnx" \
    "Liveness Detection Model"

# Anti-Spoofing Model
echo "4. Anti-Spoofing Model"
download_if_missing \
    "https://example.com/anti-spoof-model.onnx" \
    "anti_spoof.onnx" \
    "Anti-Spoofing Model"

echo ""
echo "🔧 Development Mode:"
echo "==================="
echo "For development, you can create placeholder model files:"
echo ""

# Create placeholder ONNX files for development
create_placeholder() {
    local filename=$1
    local description=$2
    
    if [ ! -f "$MODEL_DIR/$filename" ]; then
        echo "📝 Creating placeholder $filename..."
        # Create a minimal ONNX file (this is just a placeholder)
        echo "PLACEHOLDER_ONNX_MODEL_FOR_DEVELOPMENT" > "$MODEL_DIR/$filename"
        echo "   ⚠️  This is a placeholder - replace with real model for production"
    fi
}

create_placeholder "face_detection.onnx" "Face Detection"
create_placeholder "face_recognition.onnx" "Face Recognition" 
create_placeholder "liveness.onnx" "Liveness Detection"
create_placeholder "anti_spoof.onnx" "Anti-Spoofing"

echo ""
echo "✅ Model setup complete!"
echo ""
echo "📖 Next Steps:"
echo "1. Replace placeholder files with real ONNX models"
echo "2. Run 'cargo test' to verify model loading"
echo "3. Check logs for any model loading errors"
echo "4. Review models/README.md for detailed integration guide"
echo ""
echo "🔗 Useful Resources:"
echo "- InsightFace: https://github.com/deepinsight/insightface"
echo "- ONNX Model Zoo: https://github.com/onnx/models"
echo "- YOLOv5-Face: https://github.com/deepcam-cn/yolov5-face"