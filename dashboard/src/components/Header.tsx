import React from 'react'
import { useAuth } from '../contexts/AuthContext'
import { Bell, User, LogOut } from 'lucide-react'

const Header: React.FC = () => {
    const { developer, logout } = useAuth()

    return (
        <header className="bg-white shadow-sm border-b border-gray-200">
            <div className="flex items-center justify-between px-6 py-4">
                <div>
                    <h1 className="text-2xl font-semibold text-gray-900">Developer Dashboard</h1>
                    <p className="text-sm text-gray-600">Manage your OpenBank API integrations</p>
                </div>

                <div className="flex items-center space-x-4">
                    <button className="p-2 text-gray-400 hover:text-gray-600 transition-colors">
                        <Bell className="h-5 w-5" />
                    </button>

                    <div className="flex items-center space-x-3">
                        <div className="flex items-center space-x-2 px-3 py-2 rounded-lg bg-gray-50">
                            <User className="h-5 w-5 text-gray-600" />
                            <div className="text-sm">
                                <p className="font-medium text-gray-900">{developer?.name}</p>
                                <p className="text-gray-600">{developer?.email}</p>
                            </div>
                        </div>

                        <button
                            onClick={logout}
                            className="p-2 text-gray-400 hover:text-red-600 transition-colors"
                            title="Logout"
                        >
                            <LogOut className="h-5 w-5" />
                        </button>
                    </div>
                </div>
            </div>
        </header>
    )
}

export default Header