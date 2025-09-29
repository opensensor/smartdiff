# Smart Code Diff - Advanced Web UI

A state-of-the-art web interface for the Smart Code Diff engine, providing comprehensive code analysis, comparison, and visualization capabilities.

## 🚀 Features

### 📁 File Explorer & Parser
- **Drag & Drop Support**: Drop files or entire directories directly into the interface
- **Multi-format Support**: Supports 25+ programming languages including C/C++, JavaScript, Python, Java, Rust, Go, and more
- **Real-time Parsing**: Parse files instantly with progress tracking and detailed results
- **Project Structure Visualization**: Interactive file tree with language detection and file size information
- **Function Extraction**: Automatically extract and display functions, classes, and complexity metrics

### ⚖️ Visual Code Comparison
- **Side-by-side Comparison**: Compare two files with advanced diff visualization
- **Similarity Scoring**: Get detailed similarity metrics (overall, structure, content, semantic)
- **Change Detection**: Identify additions, modifications, deletions, and moves
- **Configurable Options**: Adjust similarity thresholds, ignore whitespace, detect moves
- **Real-time Results**: Instant comparison with execution time tracking

### 📊 Advanced Code Analysis
- **Multi-file Analysis**: Analyze entire codebases with cross-file detection
- **Duplicate Detection**: Find duplicate functions across files with similarity scoring
- **Complexity Metrics**: Calculate and visualize code complexity distributions
- **Dependency Analysis**: Track dependencies and relationships between files
- **Quality Insights**: Get actionable insights about code quality and maintainability

### ⚙️ Configuration Management
- **Parser Settings**: Configure file size limits, timeouts, and error recovery
- **Semantic Analysis**: Adjust resolution depth, cache size, and cross-file analysis
- **Diff Engine**: Fine-tune similarity thresholds, tree depth, and detection algorithms
- **Persistent Settings**: Save and restore configuration preferences

## 🎨 Modern Design

### User Experience
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile devices
- **Dark Mode Support**: Automatic dark mode based on system preferences
- **Accessibility**: Full keyboard navigation and screen reader support
- **Performance**: Optimized for large codebases with efficient rendering

### Visual Elements
- **Modern Typography**: Inter font family for readability and JetBrains Mono for code
- **Intuitive Icons**: Language-specific icons and visual indicators
- **Smooth Animations**: Subtle transitions and loading states
- **Color-coded Results**: Visual feedback for similarity scores and change types

## 🛠️ Technical Architecture

### Frontend Stack
- **Vanilla JavaScript**: No framework dependencies for maximum performance
- **Modern CSS**: CSS Grid, Flexbox, and CSS Custom Properties
- **Progressive Enhancement**: Works without JavaScript for basic functionality
- **Web Standards**: Uses modern web APIs for file handling and drag & drop

### API Integration
- **RESTful API**: Clean integration with the Smart Code Diff backend
- **Real-time Updates**: Live progress tracking and status updates
- **Error Handling**: Comprehensive error handling with user-friendly messages
- **Caching**: Intelligent caching of parsed results and configuration

## 🚀 Getting Started

### Prerequisites
- Smart Code Diff server running on `http://localhost:3000`
- Modern web browser (Chrome 90+, Firefox 88+, Safari 14+, Edge 90+)

### Quick Start
1. **Start the Server**:
   ```bash
   cargo run --release --bin smart-diff-server
   ```

2. **Open the Web UI**:
   Navigate to `http://localhost:3000` in your browser

3. **Upload Files**:
   - Drag and drop files or directories into the upload zone
   - Or click "Select Directory" / "Select Files" to browse

4. **Parse and Analyze**:
   - Click "Parse All Files" to extract functions and metadata
   - View results in the interactive file tree and results grid

5. **Compare Files**:
   - Switch to the "Compare" tab
   - Select source and target files from parsed results
   - Configure comparison options and click "Compare Files"

6. **Run Analysis**:
   - Switch to the "Analysis" tab
   - Enable desired analysis options
   - Click "Run Analysis" for comprehensive insights

## 📋 API Endpoints

The web UI integrates with the following API endpoints:

### Health Check
```
GET /api/health
```
Returns system status and component health information.

### File Comparison
```
POST /api/compare
```
Compare two files with detailed similarity analysis.

### Multi-file Analysis
```
POST /api/analyze
```
Analyze multiple files with cross-file detection and metrics.

### Configuration
```
POST /api/configure
```
Update system configuration settings.

## 🎯 Use Cases

### Code Review
- Compare different versions of files to understand changes
- Identify potential issues and improvements
- Track code quality metrics over time

### Refactoring Analysis
- Detect duplicate code across the codebase
- Identify opportunities for code consolidation
- Measure complexity before and after refactoring

### Migration Planning
- Analyze legacy codebases for migration planning
- Understand code structure and dependencies
- Estimate migration effort and complexity

### Quality Assurance
- Monitor code quality metrics across projects
- Identify files with high complexity or duplication
- Track improvements over time

## 🔧 Configuration Options

### Parser Configuration
- **Max File Size**: Maximum file size to parse (default: 1MB)
- **Parse Timeout**: Timeout for parsing operations (default: 30s)
- **Error Recovery**: Enable error recovery during parsing

### Semantic Analysis
- **Resolution Depth**: Maximum depth for symbol resolution (default: 10)
- **Cross-file Analysis**: Enable analysis across multiple files
- **Symbol Cache Size**: Cache size for symbol table (default: 1000)

### Diff Engine
- **Similarity Threshold**: Default threshold for similarity detection (default: 0.8)
- **Refactoring Detection**: Enable detection of refactoring patterns
- **Cross-file Tracking**: Track changes across multiple files
- **Tree Depth**: Maximum depth for AST analysis (default: 100)

## 🐛 Troubleshooting

### Common Issues

**Files not uploading**:
- Check file size limits in configuration
- Ensure files have supported extensions
- Verify browser supports File API

**Parsing failures**:
- Check server logs for detailed error messages
- Verify file encoding (UTF-8 recommended)
- Try smaller files to isolate issues

**Slow performance**:
- Reduce file count or size
- Adjust parser timeout settings
- Check system resources

### Browser Compatibility
- **Chrome 90+**: Full support
- **Firefox 88+**: Full support
- **Safari 14+**: Full support (some drag & drop limitations)
- **Edge 90+**: Full support

## 📈 Performance Tips

1. **File Organization**: Organize files in logical directory structures
2. **Batch Processing**: Parse related files together for better cross-file analysis
3. **Configuration Tuning**: Adjust settings based on your codebase size and complexity
4. **Browser Resources**: Close unnecessary tabs and applications for better performance

## 🤝 Contributing

The web UI is designed to be extensible and maintainable:

- **Modular Architecture**: Each feature is self-contained
- **Clean Code**: Well-documented and commented
- **Responsive Design**: Mobile-first approach
- **Accessibility**: WCAG 2.1 AA compliant

## 📄 License

This web UI is part of the Smart Code Diff project and follows the same licensing terms.
