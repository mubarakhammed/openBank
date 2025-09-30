import React, { useState, useEffect } from 'react'
import { useAuth } from '../contexts/AuthContext'
import {
    Plus,
    Key,
    Copy,
    Trash2,
    Globe,
    Shield,
    Eye,
    EyeOff,
    CheckCircle,
    XCircle,
    AlertCircle,
    Loader2
} from 'lucide-react'
import { toastUtils } from '../components/ToastProvider'
import TokenDisplayModal from '../components/TokenDisplayModal'

interface CreateProjectModalProps {
    isOpen: boolean
    onClose: () => void
    onSubmit: (data: any) => Promise<void>
    availableScopes: any
}

const CreateProjectModal: React.FC<CreateProjectModalProps> = ({ isOpen, onClose, onSubmit, availableScopes }) => {
    const [formData, setFormData] = useState({
        name: '',
        description: '',
        environment: 'development' as 'development' | 'staging' | 'production',
        redirect_uris: [''],
        scopes: [] as string[]
    })
    const [isSubmitting, setIsSubmitting] = useState(false)

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        setIsSubmitting(true)
        try {
            await onSubmit({
                ...formData,
                redirect_uris: formData.redirect_uris.filter(uri => uri.trim() !== '')
            })
            onClose()
            setFormData({
                name: '',
                description: '',
                environment: 'development',
                redirect_uris: [''],
                scopes: []
            })
        } catch (error) {
            // Error handling is done in the parent component
        } finally {
            setIsSubmitting(false)
        }
    }

    if (!isOpen) return null

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
                <div className="p-6 border-b border-gray-200">
                    <h2 className="text-xl font-bold text-gray-900">Create New Project</h2>
                    <p className="text-gray-600 mt-1">Set up a new API project with OAuth2 credentials</p>
                </div>

                <form onSubmit={handleSubmit} className="p-6 space-y-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Project Name *
                            </label>
                            <input
                                type="text"
                                required
                                value={formData.name}
                                onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                placeholder="My Awesome Project"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Environment *
                            </label>
                            <select
                                value={formData.environment}
                                onChange={(e) => setFormData(prev => ({ ...prev, environment: e.target.value as any }))}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            >
                                <option value="development">Development</option>
                                <option value="staging">Staging</option>
                                <option value="production">Production</option>
                            </select>
                        </div>
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Description
                        </label>
                        <textarea
                            value={formData.description}
                            onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            rows={3}
                            placeholder="Describe what this project is for..."
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Redirect URIs
                        </label>
                        {formData.redirect_uris.map((uri, index) => (
                            <div key={index} className="flex gap-2 mb-2">
                                <input
                                    type="url"
                                    value={uri}
                                    onChange={(e) => {
                                        const newUris = [...formData.redirect_uris]
                                        newUris[index] = e.target.value
                                        setFormData(prev => ({ ...prev, redirect_uris: newUris }))
                                    }}
                                    className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    placeholder="https://yourapp.com/callback"
                                />
                                {formData.redirect_uris.length > 1 && (
                                    <button
                                        type="button"
                                        onClick={() => {
                                            const newUris = formData.redirect_uris.filter((_, i) => i !== index)
                                            setFormData(prev => ({ ...prev, redirect_uris: newUris }))
                                        }}
                                        className="px-3 py-2 text-red-600 hover:bg-red-50 rounded-md"
                                    >
                                        <Trash2 className="h-4 w-4" />
                                    </button>
                                )}
                            </div>
                        ))}
                        <button
                            type="button"
                            onClick={() => setFormData(prev => ({ ...prev, redirect_uris: [...prev.redirect_uris, ''] }))}
                            className="text-sm text-blue-600 hover:text-blue-700"
                        >
                            + Add URI
                        </button>
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            API Scopes
                        </label>
                        <div className="grid grid-cols-2 md:grid-cols-3 gap-2 max-h-32 overflow-y-auto">
                            {availableScopes && Object.entries(availableScopes).map(([, scopes]: [string, any]) =>
                                Object.keys(scopes).map(scope => (
                                    <label key={scope} className="flex items-center space-x-2">
                                        <input
                                            type="checkbox"
                                            checked={formData.scopes.includes(scope)}
                                            onChange={(e) => {
                                                if (e.target.checked) {
                                                    setFormData(prev => ({ ...prev, scopes: [...prev.scopes, scope] }))
                                                } else {
                                                    setFormData(prev => ({ ...prev, scopes: prev.scopes.filter(s => s !== scope) }))
                                                }
                                            }}
                                            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        />
                                        <span className="text-sm text-gray-700">{scope}</span>
                                    </label>
                                ))
                            )}
                        </div>
                    </div>

                    <div className="flex justify-end space-x-3 pt-4 border-t border-gray-200">
                        <button
                            type="button"
                            onClick={onClose}
                            className="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            disabled={isSubmitting}
                            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed flex items-center space-x-2"
                        >
                            {isSubmitting ? (
                                <>
                                    <Loader2 className="h-4 w-4 animate-spin" />
                                    <span>Creating...</span>
                                </>
                            ) : (
                                <>
                                    <Plus className="h-4 w-4" />
                                    <span>Create Project</span>
                                </>
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    )
}

const ProjectCard: React.FC<{
    project: any
    onGenerateToken: (projectId: string) => void
    onDelete: (projectId: string) => void
    isLoading: boolean
}> = ({ project, onGenerateToken, onDelete, isLoading }) => {
    const [showCredentials, setShowCredentials] = useState(false)
    const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)

    const copyToClipboard = async (text: string, label: string) => {
        try {
            await navigator.clipboard.writeText(text)
            toastUtils.credentialsCopied(label)
        } catch (err) {
            toastUtils.error('Failed to copy to clipboard')
        }
    }

    const getEnvironmentBadgeClass = (env: string) => {
        switch (env) {
            case 'production': return 'bg-red-100 text-red-800'
            case 'staging': return 'bg-yellow-100 text-yellow-800'
            default: return 'bg-green-100 text-green-800'
        }
    }

    return (
        <div className="bg-white rounded-lg shadow-md border border-gray-200 p-6 hover:shadow-lg transition-shadow">
            <div className="flex justify-between items-start mb-4">
                <div>
                    <h3 className="text-lg font-semibold text-gray-900 mb-1">{project.name}</h3>
                    <p className="text-gray-600 text-sm mb-2">{project.description || 'No description'}</p>
                    <div className="flex items-center space-x-2">
                        <span className={`px-2 py-1 rounded-full text-xs font-medium ${getEnvironmentBadgeClass(project.environment)}`}>
                            {project.environment}
                        </span>
                        <span className={`flex items-center text-xs ${project.is_active ? 'text-green-600' : 'text-red-600'}`}>
                            {project.is_active ? <CheckCircle className="h-3 w-3 mr-1" /> : <XCircle className="h-3 w-3 mr-1" />}
                            {project.is_active ? 'Active' : 'Inactive'}
                        </span>
                    </div>
                </div>
                <div className="flex space-x-2">
                    <button
                        onClick={() => onGenerateToken(project.id)}
                        disabled={isLoading}
                        className="p-2 text-blue-600 hover:bg-blue-50 rounded-md transition-colors disabled:opacity-50"
                        title="Generate API Token"
                    >
                        <Key className="h-4 w-4" />
                    </button>
                    <button
                        onClick={() => setShowDeleteConfirm(true)}
                        disabled={isLoading}
                        className="p-2 text-red-600 hover:bg-red-50 rounded-md transition-colors disabled:opacity-50"
                        title="Delete Project"
                    >
                        <Trash2 className="h-4 w-4" />
                    </button>
                </div>
            </div>

            <div className="space-y-3">
                <div>
                    <label className="block text-xs font-medium text-gray-500 mb-1">CLIENT CREDENTIALS</label>
                    <div className="flex items-center space-x-2">
                        <code className="flex-1 px-2 py-1 bg-gray-100 rounded text-xs font-mono">
                            {showCredentials ? project.client_id : '••••••••••••••••••••••••••••••••••••••••'}
                        </code>
                        <button
                            onClick={() => setShowCredentials(!showCredentials)}
                            className="p-1 text-gray-500 hover:text-gray-700"
                            title={showCredentials ? 'Hide credentials' : 'Show credentials'}
                        >
                            {showCredentials ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                        </button>
                        {showCredentials && (
                            <button
                                onClick={() => copyToClipboard(project.client_id, 'Client credentials')}
                                className="p-1 text-gray-500 hover:text-gray-700"
                                title="Copy credentials"
                            >
                                <Copy className="h-4 w-4" />
                            </button>
                        )}
                    </div>
                </div>

                <div>
                    <label className="block text-xs font-medium text-gray-500 mb-1">SCOPES</label>
                    <div className="flex flex-wrap gap-1">
                        {project.scopes.map((scope: string) => (
                            <span key={scope} className="px-2 py-1 bg-gray-100 text-gray-700 rounded text-xs">
                                {scope}
                            </span>
                        ))}
                    </div>
                </div>

                <div>
                    <label className="block text-xs font-medium text-gray-500 mb-1">REDIRECT URIS</label>
                    <div className="space-y-1">
                        {project.redirect_uris.map((uri: string, index: number) => (
                            <div key={index} className="flex items-center space-x-2">
                                <Globe className="h-3 w-3 text-gray-400" />
                                <code className="text-xs text-gray-600 font-mono">{uri}</code>
                            </div>
                        ))}
                    </div>
                </div>

                <div className="text-xs text-gray-500">
                    Created {new Date(project.created_at).toLocaleDateString()}
                </div>
            </div>

            {/* Delete Confirmation Modal */}
            {showDeleteConfirm && (
                <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
                    <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
                        <div className="flex items-center space-x-3 mb-4">
                            <AlertCircle className="h-6 w-6 text-red-600" />
                            <h3 className="text-lg font-semibold text-gray-900">Delete Project</h3>
                        </div>
                        <p className="text-gray-600 mb-6">
                            Are you sure you want to delete "{project.name}"? This action cannot be undone and will invalidate all associated API tokens.
                        </p>
                        <div className="flex justify-end space-x-3">
                            <button
                                onClick={() => setShowDeleteConfirm(false)}
                                className="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                            >
                                Cancel
                            </button>
                            <button
                                onClick={() => {
                                    onDelete(project.id)
                                    setShowDeleteConfirm(false)
                                }}
                                disabled={isLoading}
                                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 disabled:opacity-50 flex items-center space-x-2"
                            >
                                {isLoading ? (
                                    <>
                                        <Loader2 className="h-4 w-4 animate-spin" />
                                        <span>Deleting...</span>
                                    </>
                                ) : (
                                    <>
                                        <Trash2 className="h-4 w-4" />
                                        <span>Delete</span>
                                    </>
                                )}
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    )
}

const Projects: React.FC = () => {
    const {
        projects,
        isProjectsLoading,
        isLoading,
        error,
        createProject,
        deleteProject,
        generateToken,
        availableScopes,
        loadAvailableScopes
    } = useAuth()
    const [showCreateModal, setShowCreateModal] = useState(false)
    const [tokenModalData, setTokenModalData] = useState<{ tokenData: any, projectName: string } | null>(null)

    useEffect(() => {
        // Load mock data for projects and scopes
        loadAvailableScopes()
    }, [])

    const handleCreateProject = async (data: any) => {
        await createProject(data)
        setShowCreateModal(false)
    }

    const handleGenerateToken = async (projectId: string) => {
        try {
            const project = projects.find(p => p.id === projectId)
            const tokenData = await generateToken(projectId)
            setTokenModalData({
                tokenData,
                projectName: project?.name || 'Unknown Project'
            })
        } catch (error) {
            // Error is handled in the context
        }
    }

    const handleDeleteProject = async (projectId: string) => {
        try {
            await deleteProject(projectId)
        } catch (error) {
            // Error is handled in the context
        }
    }

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <div>
                    <h1 className="text-2xl font-bold text-gray-900">Projects</h1>
                    <p className="text-gray-600 mt-1">Manage your API projects and OAuth2 credentials</p>
                </div>
                <button
                    onClick={() => setShowCreateModal(true)}
                    className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                    <Plus className="h-4 w-4" />
                    <span>New Project</span>
                </button>
            </div>

            {error && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4">
                    <div className="flex items-center space-x-2">
                        <XCircle className="h-5 w-5 text-red-500" />
                        <span className="text-red-800">{error}</span>
                    </div>
                </div>
            )}

            {isProjectsLoading ? (
                <div className="flex items-center justify-center py-12">
                    <div className="flex items-center space-x-2 text-gray-600">
                        <Loader2 className="h-5 w-5 animate-spin" />
                        <span>Loading projects...</span>
                    </div>
                </div>
            ) : projects.length === 0 ? (
                <div className="text-center py-12">
                    <Shield className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                    <h3 className="text-lg font-medium text-gray-900 mb-2">No projects yet</h3>
                    <p className="text-gray-600 mb-4">Create your first API project to get started with OpenBank APIs</p>
                    <button
                        onClick={() => setShowCreateModal(true)}
                        className="inline-flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                    >
                        <Plus className="h-4 w-4" />
                        <span>Create Your First Project</span>
                    </button>
                </div>
            ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {projects.map((project) => (
                        <ProjectCard
                            key={project.id}
                            project={project}
                            onGenerateToken={handleGenerateToken}
                            onDelete={handleDeleteProject}
                            isLoading={isLoading}
                        />
                    ))}
                </div>
            )}

            <CreateProjectModal
                isOpen={showCreateModal}
                onClose={() => setShowCreateModal(false)}
                onSubmit={handleCreateProject}
                availableScopes={availableScopes}
            />

            <TokenDisplayModal
                isOpen={tokenModalData !== null}
                onClose={() => setTokenModalData(null)}
                tokenData={tokenModalData?.tokenData || null}
                projectName={tokenModalData?.projectName || ''}
            />
        </div>
    )
}

export default Projects