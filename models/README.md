# OpenBank Identity ML Models

This directory contains the machine learning models used by the OpenBank Identity verification system.

## Required Models

### 1. Face Detection Model
- **File**: `face_detection.onnx`
- **Purpose**: Detect and locate faces in images
- **Input**: RGB image (640x640)
- **Output**: Bounding boxes and confidence scores
- **Recommended**: YOLOv5-face or MTCNN converted to ONNX

### 2. Face Recognition Model
- **File**: `face_recognition.onnx` 
- **Purpose**: Extract 512-dimensional face embeddings
- **Input**: Cropped face image (112x112)
- **Output**: 512-dimensional embedding vector
- **Recommended**: ArcFace, CosFace, or InsightFace models

### 3. Liveness Detection Model
- **File**: `liveness.onnx`
- **Purpose**: Detect if face is from a live person vs photo/video
- **Input**: Face image (112x112)
- **Output**: Liveness score (0-1)
- **Recommended**: FAS (Face Anti-Spoofing) models

### 4. Anti-Spoofing Model
- **File**: `anti_spoof.onnx`
- **Purpose**: Advanced spoofing detection (masks, deepfakes, etc.)
- **Input**: Face image (112x112) 
- **Output**: Real/fake classification and confidence
- **Recommended**: Multi-modal anti-spoofing models

## Model Sources

### Open Source Options:
1. **InsightFace**: https://github.com/deepinsight/insightface
   - Pre-trained face recognition models
   - ONNX format available
   - High accuracy and performance

2. **FaceX-Zoo**: https://github.com/JDAI-CV/FaceX-Zoo
   - Collection of face analysis models
   - Multiple architectures available

3. **Silent-Face-Anti-Spoofing**: https://github.com/minivision-ai/Silent-Face-Anti-Spoofing
   - Lightweight anti-spoofing models
   - Real-time performance

### Commercial Options:
1. **AWS Rekognition**: Export custom models
2. **Azure Face API**: Custom model training
3. **Google Cloud Vision**: AutoML Face models

## Integration Steps

1. **Download/Train Models**: Get ONNX format models
2. **Place in Directory**: Copy models to this `/models` folder
3. **Update Paths**: Verify paths in `ModelConfig` match filenames
4. **Test Loading**: Run `cargo test` to verify models load correctly
5. **Performance Tuning**: Adjust input sizes and thresholds as needed

## Model Evaluation

### Accuracy Metrics:
- **Face Detection**: mAP@0.5 > 0.95
- **Face Recognition**: TAR@FAR=0.1% > 0.99
- **Liveness Detection**: EER < 5%
- **Anti-Spoofing**: ACER < 5%

### Performance Targets:
- **Inference Time**: < 200ms per image (CPU)
- **Memory Usage**: < 500MB total model size
- **Throughput**: > 100 verifications/minute

## Security Considerations

1. **Model Integrity**: Verify checksums of downloaded models
2. **Access Control**: Restrict file permissions to application user
3. **Encryption**: Consider encrypting models at rest
4. **Versioning**: Track model versions for audit trails
5. **Updates**: Plan for secure model update procedures

## Troubleshooting

### Common Issues:
1. **Model Not Found**: Check file paths and permissions
2. **ONNX Runtime Error**: Verify model format and dependencies
3. **Poor Accuracy**: Check input preprocessing and normalization
4. **Slow Performance**: Consider model quantization or GPU acceleration

### Support:
- Check logs in `/logs/identity-service.log`
- Enable debug logging with `RUST_LOG=debug`
- Test with sample images in `/tests/fixtures/`