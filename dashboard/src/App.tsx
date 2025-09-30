import { Routes, Route } from 'react-router-dom'
import { AuthProvider } from './contexts/AuthContext'
import ToastProvider from './components/ToastProvider'
import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import Login from './pages/Login'
import Projects from './pages/Projects'
import APIExplorer from './pages/APIExplorer'
import Documentation from './pages/Documentation'
import Settings from './pages/Settings'

function App() {
    return (
        <>
            <AuthProvider>
                <Routes>
                    <Route path="/login" element={<Login />} />
                    <Route path="/" element={<Layout />}>
                        <Route index element={<Dashboard />} />
                        <Route path="projects" element={<Projects />} />
                        <Route path="api-explorer" element={<APIExplorer />} />
                        <Route path="documentation" element={<Documentation />} />
                        <Route path="settings" element={<Settings />} />
                    </Route>
                </Routes>
            </AuthProvider>
            <ToastProvider />
        </>
    )
}

export default App