import React from 'react'
import { Link, useLocation } from 'react-router-dom'
import { Home, FolderOpen, Code, BookOpen, Settings, Shield } from 'lucide-react'

const navigation = [
    { name: 'Dashboard', href: '/', icon: Home },
    { name: 'Projects', href: '/projects', icon: FolderOpen },
    { name: 'API Explorer', href: '/api-explorer', icon: Code },
    { name: 'Documentation', href: '/documentation', icon: BookOpen },
    { name: 'Settings', href: '/settings', icon: Settings },
]

const Sidebar: React.FC = () => {
    const location = useLocation()

    return (
        <div className="bg-gray-900 text-white w-64 flex-shrink-0">
            <div className="flex items-center px-6 py-4 border-b border-gray-700">
                <Shield className="h-8 w-8 text-blue-400" />
                <h1 className="ml-3 text-xl font-bold">OpenBank</h1>
            </div>

            <nav className="mt-8">
                <div className="px-4">
                    {navigation.map((item) => {
                        const isActive = location.pathname === item.href
                        return (
                            <Link
                                key={item.name}
                                to={item.href}
                                className={`group flex items-center px-2 py-2 text-sm font-medium rounded-md mb-2 transition-colors ${isActive
                                        ? 'bg-blue-600 text-white'
                                        : 'text-gray-300 hover:bg-gray-700 hover:text-white'
                                    }`}
                            >
                                <item.icon
                                    className={`mr-3 h-5 w-5 ${isActive ? 'text-white' : 'text-gray-400 group-hover:text-white'
                                        }`}
                                />
                                {item.name}
                            </Link>
                        )
                    })}
                </div>
            </nav>
        </div>
    )
}

export default Sidebar