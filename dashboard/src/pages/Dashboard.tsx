import React from 'react'
import { useAuth } from '../contexts/AuthContext'
import { BarChart3, Users, Code, Activity, Shield, Globe } from 'lucide-react'

const Dashboard: React.FC = () => {
    const { developer } = useAuth()

    const stats = [
        { name: 'API Calls Today', value: '2,847', icon: BarChart3, change: '+12%', changeType: 'positive' },
        { name: 'Active Projects', value: '3', icon: Code, change: '+1', changeType: 'positive' },
        { name: 'Success Rate', value: '99.9%', icon: Activity, change: '+0.1%', changeType: 'positive' },
        { name: 'Response Time', value: '45ms', icon: Globe, change: '-2ms', changeType: 'positive' },
    ]

    const recentActivity = [
        { action: 'Token generated', project: 'Production API', time: '2 minutes ago', status: 'success' },
        { action: 'API call', project: 'Staging API', time: '5 minutes ago', status: 'success' },
        { action: 'Project created', project: 'Development API', time: '1 hour ago', status: 'success' },
        { action: 'Token refreshed', project: 'Production API', time: '2 hours ago', status: 'success' },
    ]

    return (
        <div className="space-y-6">
            {/* Welcome Header */}
            <div className="bg-gradient-to-r from-blue-600 to-indigo-600 rounded-lg p-6 text-white">
                <div className="flex items-center">
                    <Shield className="h-12 w-12 text-blue-200" />
                    <div className="ml-4">
                        <h1 className="text-2xl font-bold">Welcome back, {developer?.name}!</h1>
                        <p className="text-blue-100">
                            Here's what's happening with your OpenBank API integrations today.
                        </p>
                    </div>
                </div>
            </div>

            {/* Stats Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {stats.map((stat) => (
                    <div key={stat.name} className="bg-white rounded-lg p-6 shadow-sm border border-gray-200">
                        <div className="flex items-center">
                            <div className="flex-shrink-0">
                                <stat.icon className="h-8 w-8 text-blue-600" />
                            </div>
                            <div className="ml-4 w-0 flex-1">
                                <dl>
                                    <dt className="text-sm font-medium text-gray-500 truncate">{stat.name}</dt>
                                    <dd className="flex items-baseline">
                                        <div className="text-2xl font-semibold text-gray-900">{stat.value}</div>
                                        <div className={`ml-2 flex items-baseline text-sm font-semibold ${stat.changeType === 'positive' ? 'text-green-600' : 'text-red-600'
                                            }`}>
                                            {stat.change}
                                        </div>
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            {/* Recent Activity */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                    <div className="px-6 py-4 border-b border-gray-200">
                        <h3 className="text-lg font-medium text-gray-900">Recent Activity</h3>
                    </div>
                    <div className="px-6 py-4">
                        <div className="space-y-4">
                            {recentActivity.map((activity, index) => (
                                <div key={index} className="flex items-center space-x-3">
                                    <div className={`flex-shrink-0 w-2 h-2 rounded-full ${activity.status === 'success' ? 'bg-green-400' : 'bg-red-400'
                                        }`} />
                                    <div className="flex-1 min-w-0">
                                        <p className="text-sm font-medium text-gray-900">
                                            {activity.action}
                                        </p>
                                        <p className="text-sm text-gray-500">
                                            {activity.project} • {activity.time}
                                        </p>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* Quick Actions */}
                <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                    <div className="px-6 py-4 border-b border-gray-200">
                        <h3 className="text-lg font-medium text-gray-900">Quick Actions</h3>
                    </div>
                    <div className="px-6 py-4">
                        <div className="space-y-3">
                            <button className="w-full text-left p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div className="flex items-center">
                                    <Code className="h-5 w-5 text-blue-600" />
                                    <span className="ml-3 text-sm font-medium">Create New Project</span>
                                </div>
                            </button>
                            <button className="w-full text-left p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div className="flex items-center">
                                    <Shield className="h-5 w-5 text-green-600" />
                                    <span className="ml-3 text-sm font-medium">Generate API Token</span>
                                </div>
                            </button>
                            <button className="w-full text-left p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <div className="flex items-center">
                                    <Users className="h-5 w-5 text-purple-600" />
                                    <span className="ml-3 text-sm font-medium">View API Documentation</span>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Dashboard