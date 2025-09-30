# OpenBank Developer Dashboard

A modern, sleek React dashboard for developers to manage their OpenBank API integrations. Built with React, TypeScript, Tailwind CSS, and Vite for optimal performance and developer experience.

## Features

### 🔐 Authentication
- Secure developer login system
- JWT token management with automatic refresh
- Protected routes and role-based access

### 📊 Dashboard Overview
- Real-time API usage statistics
- Performance metrics and monitoring
- Recent activity tracking
- Quick action shortcuts

### 🚀 Project Management
- Create and manage API projects
- Environment-specific configurations (dev, staging, production)
- Client credential generation and management
- Project-level scope and permission control

### 🔍 API Explorer
- Interactive API testing interface
- Live endpoint documentation
- Request/response inspection
- Authentication testing tools

### 📚 Documentation
- Comprehensive API reference
- Integration guides and tutorials
- Code examples in multiple languages
- Best practices and troubleshooting

### ⚙️ Settings & Configuration
- Developer profile management
- Security settings and preferences
- Notification configuration
- API key management

## Tech Stack

- **Frontend**: React 18 with TypeScript
- **Styling**: Tailwind CSS with custom design system
- **Build Tool**: Vite for fast development and building
- **State Management**: React Query for server state
- **Routing**: React Router v6
- **Icons**: Lucide React
- **Forms**: React Hook Form with Zod validation
- **Notifications**: React Hot Toast

## Getting Started

### Prerequisites
- Node.js 18+ and npm
- OpenBank API server running on `http://127.0.0.1:8080`

### Installation

1. Navigate to the dashboard directory:
```bash
cd dashboard
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

4. Open your browser and navigate to `http://localhost:3000`

### Quick Setup Script
```bash
chmod +x setup.sh
./setup.sh
```

## Development

### Available Scripts

- `npm run dev` - Start development server with hot reload
- `npm run build` - Build for production
- `npm run preview` - Preview production build locally
- `npm run lint` - Run ESLint for code quality
- `npm run type-check` - Run TypeScript type checking

### Project Structure

```
dashboard/
├── src/
│   ├── components/         # Reusable UI components
│   │   ├── Layout.tsx     # Main layout wrapper
│   │   ├── Sidebar.tsx    # Navigation sidebar
│   │   └── Header.tsx     # Top header with user info
│   ├── contexts/          # React contexts
│   │   └── AuthContext.tsx # Authentication state management
│   ├── pages/             # Route components
│   │   ├── Login.tsx      # Authentication page
│   │   ├── Dashboard.tsx  # Main dashboard overview
│   │   ├── Projects.tsx   # Project management
│   │   ├── APIExplorer.tsx # API testing interface
│   │   ├── Documentation.tsx # API docs
│   │   └── Settings.tsx   # User settings
│   ├── lib/               # Utility functions
│   │   └── utils.ts       # Common utilities
│   ├── App.tsx            # Main app component
│   ├── main.tsx           # Application entry point
│   └── index.css          # Global styles
├── public/                # Static assets
├── package.json           # Dependencies and scripts
├── vite.config.ts         # Vite configuration
├── tailwind.config.js     # Tailwind CSS configuration
└── tsconfig.json          # TypeScript configuration
```

## API Integration

The dashboard integrates with your OpenBank API server running on `http://127.0.0.1:8080`. All API calls are proxied through Vite's development server:

- Frontend: `http://localhost:3000`
- API Proxy: `/api/*` → `http://127.0.0.1:8080/*`

### Authentication Flow

1. **Login**: Developer enters credentials
2. **Token Storage**: JWT tokens stored securely in localStorage
3. **Auto-Refresh**: Tokens automatically refreshed before expiration
4. **Protected Routes**: Unauthenticated users redirected to login

## Design System

### Color Palette
- **Primary**: Blue (#3B82F6) - Main brand color
- **Secondary**: Indigo (#6366F1) - Accent color
- **Success**: Green (#10B981) - Success states
- **Warning**: Yellow (#F59E0B) - Warning states
- **Error**: Red (#EF4444) - Error states
- **Neutral**: Gray scale for text and backgrounds

### Typography
- **Headings**: Inter font, bold weights
- **Body**: Inter font, regular and medium weights
- **Code**: Mono font for code snippets

### Components
All components follow consistent design patterns:
- Rounded corners (0.5rem default)
- Subtle shadows and borders
- Smooth transitions and hover effects
- Responsive design for all screen sizes

## Security Features

- **JWT Token Handling**: Secure storage and automatic refresh
- **Route Protection**: Authenticated routes with automatic redirects
- **API Security**: All requests include proper authentication headers
- **XSS Protection**: Sanitized data rendering and secure practices

## Performance Optimizations

- **Code Splitting**: Automatic route-based code splitting
- **Asset Optimization**: Vite's built-in optimization for CSS/JS
- **Lazy Loading**: Components and routes loaded on demand
- **Caching**: React Query for intelligent API response caching

## Browser Compatibility

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Contributing

1. Follow the existing code style and patterns
2. Use TypeScript for all new components
3. Ensure responsive design for mobile and desktop
4. Add proper error handling and loading states
5. Update documentation for new features

## License

This project is part of the OpenBank ecosystem and follows the same licensing terms.