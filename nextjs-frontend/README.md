# Smart Diff - Next.js Frontend

A world-class Next.js application for advanced code comparison with graph-based function matching and intelligent diff visualization.

## Features

- 🚀 **Next.js 15** with App Router
- 🎨 **Tailwind CSS** for styling
- 📱 **Responsive Design** with modern UI components
- 🔍 **File System Explorer** for OS directory browsing
- 📊 **Graph-based Function Matching** visualization
- 🔄 **Real-time Collaboration** features
- ⚡ **Performance Optimized** for large codebases
- 🎯 **TypeScript** for type safety

## Architecture

### Directory Structure

```
src/
├── app/                    # Next.js App Router
│   ├── layout.tsx         # Root layout
│   ├── page.tsx           # Home page
│   ├── globals.css        # Global styles
│   └── providers.tsx      # React Query provider
├── components/            # React components
│   ├── ui/               # Base UI components
│   ├── layout/           # Layout components
│   ├── filesystem/       # File system components
│   ├── diff/             # Diff viewer components
│   ├── graph/            # Graph visualization
│   └── visualization/    # Advanced visualizations
├── lib/                  # Utility libraries
├── hooks/                # Custom React hooks
├── types/                # TypeScript type definitions
├── utils/                # Utility functions
├── store/                # State management
└── api/                  # API client
```

### Technology Stack

- **Framework**: Next.js 15 with App Router
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State Management**: Zustand
- **Data Fetching**: TanStack Query (React Query)
- **HTTP Client**: Axios
- **Graph Visualization**: React Flow + D3.js
- **Code Editor**: Monaco Editor
- **Real-time**: Socket.IO
- **Forms**: React Hook Form + Zod
- **Icons**: Lucide React

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn
- Running Rust backend on port 3000

### Installation

1. Install dependencies:
```bash
npm install
```

2. Set up environment variables:
```bash
cp .env.local.example .env.local
```

3. Start the development server:
```bash
npm run dev
```

4. Open [http://localhost:3001](http://localhost:3001) in your browser.

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint
- `npm run type-check` - Run TypeScript type checking

## API Integration

The frontend communicates with the Rust backend via REST APIs:

- **File System**: Browse directories, read files, search
- **Diff Engine**: Compare files/directories with graph matching
- **Analysis**: Function analysis, dependency graphs
- **Language Detection**: Automatic language detection

## Key Components

### File System Explorer
- Tree view of OS directories
- Multi-selection support
- Drag-and-drop functionality
- Virtual scrolling for performance

### Graph-based Diff Viewer
- Function-level comparison
- Position-independent matching
- Move/rename detection
- Interactive graph visualization

### Advanced Visualizations
- Dependency graphs with D3.js
- Function relationship mapping
- Similarity heatmaps
- Interactive filtering

## Performance Optimizations

- **Virtual Scrolling** for large file lists
- **Lazy Loading** of components and data
- **Caching** with React Query
- **Code Splitting** with Next.js
- **Progressive Loading** for large codebases

## Development

### Code Style

- ESLint + Prettier for code formatting
- TypeScript strict mode enabled
- Tailwind CSS for consistent styling
- Component-based architecture

### State Management

- Zustand for global state
- React Query for server state
- Local state with React hooks

### Testing

```bash
npm run test        # Run tests
npm run test:watch  # Watch mode
npm run test:coverage # Coverage report
```

## Deployment

### Production Build

```bash
npm run build
npm run start
```

### Docker

```bash
docker build -t smart-diff-frontend .
docker run -p 3001:3001 smart-diff-frontend
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
