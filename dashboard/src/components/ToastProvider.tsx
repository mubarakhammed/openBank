import React from 'react'
import { Toaster } from 'react-hot-toast'

const ToastProvider: React.FC = () => {
    return (
        <Toaster
            position="top-right"
            reverseOrder={false}
            gutter={8}
            containerClassName=""
            containerStyle={{}}
            toastOptions={{
                className: '',
                duration: 4000,
                style: {
                    background: '#fff',
                    color: '#374151',
                    borderRadius: '8px',
                    border: '1px solid #e5e7eb',
                    boxShadow: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
                    fontSize: '14px',
                    fontWeight: '500',
                    padding: '12px 16px',
                    maxWidth: '500px',
                },
                success: {
                    duration: 3000,
                    style: {
                        border: '1px solid #10b981',
                        color: '#059669',
                    },
                    iconTheme: {
                        primary: '#10b981',
                        secondary: '#ffffff',
                    },
                },
                error: {
                    duration: 5000,
                    style: {
                        border: '1px solid #ef4444',
                        color: '#dc2626',
                    },
                    iconTheme: {
                        primary: '#ef4444',
                        secondary: '#ffffff',
                    },
                },
                loading: {
                    duration: Infinity,
                    style: {
                        border: '1px solid #3b82f6',
                        color: '#2563eb',
                    },
                    iconTheme: {
                        primary: '#3b82f6',
                        secondary: '#ffffff',
                    },
                },
            }}
        />
    )
}

export default ToastProvider

import toast from 'react-hot-toast'

// Enhanced toast utilities with better UX
export const toastUtils = {
    success: (message: string, options?: any) => {
        return toast.success(message, {
            ...options,
            icon: '🎉',
        })
    },

    error: (message: string, options?: any) => {
        return toast.error(message, {
            ...options,
            icon: '❌',
        })
    },

    loading: (message: string, options?: any) => {
        return toast.loading(message, {
            ...options,
            icon: '⏳',
        })
    },

    custom: (content: string, options?: any) => {
        return toast.custom(content, options)
    },

    dismiss: (toastId?: string) => {
        return toast.dismiss(toastId)
    },

    // Project-specific toasts
    projectCreated: (projectName: string) => {
        return toast.success(`🚀 Project "${projectName}" created successfully!`, {
            duration: 4000,
        })
    },

    projectDeleted: (projectName: string) => {
        return toast.success(`🗑️ Project "${projectName}" deleted successfully!`, {
            duration: 3000,
        })
    },

    tokenGenerated: () => {
        return toast.success('🔑 API token generated successfully!', {
            duration: 4000,
        })
    },

    credentialsCopied: (type: string) => {
        return toast.success(`📋 ${type} copied to clipboard!`, {
            duration: 2000,
        })
    },

    authenticationSuccess: (developerName: string) => {
        return toast.success(`👋 Welcome back, ${developerName}!`, {
            duration: 3000,
        })
    },

    logoutSuccess: () => {
        return toast.success('👋 Logged out successfully!', {
            duration: 2000,
        })
    }
}