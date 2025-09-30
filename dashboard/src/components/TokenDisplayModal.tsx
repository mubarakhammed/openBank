import React, { useState } from 'react'
import { Copy, Eye, EyeOff, X, CheckCircle, AlertTriangle, Clock } from 'lucide-react'
import { toastUtils } from './ToastProvider'

interface TokenDisplayModalProps {
    isOpen: boolean
    onClose: () => void
    tokenData: {
        access_token: string
        token_type: string
        expires_in: number
        scope?: string
    } | null
    projectName: string
}

const TokenDisplayModal: React.FC<TokenDisplayModalProps> = ({
    isOpen,
    onClose,
    tokenData,
    projectName
}) => {
    const [showToken, setShowToken] = useState(false)

    const copyToClipboard = async (text: string, label: string) => {
        try {
            await navigator.clipboard.writeText(text)
            toastUtils.credentialsCopied(label)
        } catch (err) {
            toastUtils.error('Failed to copy to clipboard')
        }
    }

    const formatExpiryTime = (expiresIn: number) => {
        const hours = Math.floor(expiresIn / 3600)
        const minutes = Math.floor((expiresIn % 3600) / 60)

        if (hours > 0) {
            return `${hours}h ${minutes}m`
        } else {
            return `${minutes}m`
        }
    }

    const getExpiryDate = (expiresIn: number) => {
        const expiryDate = new Date(Date.now() + expiresIn * 1000)
        return expiryDate.toLocaleString()
    }

    if (!isOpen || !tokenData) return null

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
                <div className="p-6 border-b border-gray-200">
                    <div className="flex justify-between items-start">
                        <div>
                            <h2 className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                                <CheckCircle className="h-6 w-6 text-green-500" />
                                <span>API Token Generated</span>
                            </h2>
                            <p className="text-gray-600 mt-1">
                                Your access token for <span className="font-medium">{projectName}</span>
                            </p>
                        </div>
                        <button
                            onClick={onClose}
                            className="p-2 text-gray-400 hover:text-gray-600 rounded-md hover:bg-gray-100"
                        >
                            <X className="h-5 w-5" />
                        </button>
                    </div>
                </div>

                <div className="p-6 space-y-6">
                    {/* Security Warning */}
                    <div className="bg-amber-50 border border-amber-200 rounded-lg p-4">
                        <div className="flex items-start space-x-3">
                            <AlertTriangle className="h-5 w-5 text-amber-600 mt-0.5" />
                            <div>
                                <h3 className="text-sm font-medium text-amber-800">Security Notice</h3>
                                <p className="text-sm text-amber-700 mt-1">
                                    This token will only be shown once. Store it securely and never share it publicly.
                                    Treat it like a password.
                                </p>
                            </div>
                        </div>
                    </div>

                    {/* Token Details */}
                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Access Token
                            </label>
                            <div className="flex items-center space-x-2">
                                <div className="flex-1 relative">
                                    <code className="block w-full px-3 py-2 pr-20 bg-gray-100 border border-gray-300 rounded-md text-sm font-mono break-all">
                                        {showToken ? tokenData.access_token : '•'.repeat(50)}
                                    </code>
                                    <div className="absolute right-2 top-1/2 transform -translate-y-1/2 flex space-x-1">
                                        <button
                                            onClick={() => setShowToken(!showToken)}
                                            className="p-1 text-gray-500 hover:text-gray-700"
                                            title={showToken ? 'Hide token' : 'Show token'}
                                        >
                                            {showToken ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                                        </button>
                                        <button
                                            onClick={() => copyToClipboard(tokenData.access_token, 'Access token')}
                                            className="p-1 text-gray-500 hover:text-gray-700"
                                            title="Copy token"
                                        >
                                            <Copy className="h-4 w-4" />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    Token Type
                                </label>
                                <div className="flex items-center space-x-2">
                                    <code className="px-3 py-2 bg-gray-100 border border-gray-300 rounded-md text-sm">
                                        {tokenData.token_type}
                                    </code>
                                    <button
                                        onClick={() => copyToClipboard(tokenData.token_type, 'Token type')}
                                        className="p-1 text-gray-500 hover:text-gray-700"
                                        title="Copy token type"
                                    >
                                        <Copy className="h-4 w-4" />
                                    </button>
                                </div>
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    Expires In
                                </label>
                                <div className="flex items-center space-x-2 text-sm text-gray-600">
                                    <Clock className="h-4 w-4" />
                                    <span>{formatExpiryTime(tokenData.expires_in)}</span>
                                    <span className="text-xs text-gray-500">
                                        ({getExpiryDate(tokenData.expires_in)})
                                    </span>
                                </div>
                            </div>
                        </div>

                        {tokenData.scope && (
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    Scopes
                                </label>
                                <div className="flex flex-wrap gap-2">
                                    {tokenData.scope.split(' ').map((scope) => (
                                        <span
                                            key={scope}
                                            className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full"
                                        >
                                            {scope}
                                        </span>
                                    ))}
                                </div>
                            </div>
                        )}
                    </div>

                    {/* Usage Example */}
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Usage Example
                        </label>
                        <div className="bg-gray-900 text-gray-100 rounded-md p-4 text-sm font-mono overflow-x-auto">
                            <div className="text-green-400"># Using curl</div>
                            <div className="mt-1">
                                curl -H "Authorization: {tokenData.token_type} {showToken ? tokenData.access_token : '[YOUR_TOKEN]'}" \\
                            </div>
                            <div className="ml-4">
                                https://api.openbank.com/v1/identity/verify
                            </div>
                            <div className="mt-3 text-green-400"># Using JavaScript</div>
                            <div className="mt-1">
                                const response = await fetch('https://api.openbank.com/v1/identity/verify', {'{'}
                            </div>
                            <div className="ml-4">
                                headers: {'{'}
                            </div>
                            <div className="ml-8">
                                'Authorization': '{tokenData.token_type} {showToken ? tokenData.access_token : '[YOUR_TOKEN]'}'
                            </div>
                            <div className="ml-4">
                                {'}'}
                            </div>
                            <div>
                                {'}'});
                            </div>
                        </div>
                        <button
                            onClick={() => copyToClipboard(
                                `curl -H "Authorization: ${tokenData.token_type} ${tokenData.access_token}" https://api.openbank.com/v1/identity/verify`,
                                'cURL example'
                            )}
                            className="mt-2 text-sm text-blue-600 hover:text-blue-700"
                        >
                            📋 Copy cURL example
                        </button>
                    </div>

                    {/* Action Buttons */}
                    <div className="flex justify-end space-x-3 pt-4 border-t border-gray-200">
                        <button
                            onClick={() => copyToClipboard(tokenData.access_token, 'Access token')}
                            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 flex items-center space-x-2"
                        >
                            <Copy className="h-4 w-4" />
                            <span>Copy Token</span>
                        </button>
                        <button
                            onClick={onClose}
                            className="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        >
                            Done
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default TokenDisplayModal