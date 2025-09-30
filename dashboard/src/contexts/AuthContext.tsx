import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import toast from 'react-hot-toast'
import { toastUtils } from '../components/ToastProvider'
import { apiClient, type CreateProjectRequest } from '../lib/api'

export interface Developer {
    id: string
    name: string
    email: string
    company?: string
    title?: string
    created_at: string
}

export interface Project {
    id: string
    name: string
    description?: string
    environment: 'development' | 'staging' | 'production'
    client_id: string
    redirect_uris: string[]
    scopes: string[]
    is_active: boolean
    created_at: string
}

interface AuthContextType {
    developer: Developer | null
    projects: Project[]
    accessToken: string | null
    availableScopes: any
    login: (email: string, password: string) => Promise<void>
    logout: () => void
    refreshToken: () => Promise<void>
    createProject: (data: CreateProjectRequest) => Promise<Project>
    updateProject: (projectId: string, data: any) => Promise<Project>
    deleteProject: (projectId: string) => Promise<void>
    loadProjects: () => Promise<void>
    generateToken: (projectId: string, scopes?: string[]) => Promise<any>
    refreshProjectToken: (projectId: string, jti: string) => Promise<any>
    loadAvailableScopes: () => Promise<void>
    isLoading: boolean
    isProjectsLoading: boolean
    error: string | null
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export const useAuth = () => {
    const context = useContext(AuthContext)
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider')
    }
    return context
}

interface AuthProviderProps {
    children: ReactNode
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
    const [developer, setDeveloper] = useState<Developer | null>(null)
    const [projects, setProjects] = useState<Project[]>([])
    const [availableScopes, setAvailableScopes] = useState<any>(null)
    const [accessToken, setAccessToken] = useState<string | null>(
        localStorage.getItem('openbank_token')
    )
    const [isLoading, setIsLoading] = useState(false)
    const [isProjectsLoading, setIsProjectsLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)

    const login = async (email: string, password: string) => {
        setIsLoading(true)
        try {
            const response = await apiClient.login({ email, password })

            if (response.status === 'success' && response.data) {
                const { developer: devData, access_token } = response.data

                const developer: Developer = {
                    id: devData.id,
                    name: devData.name,
                    email: devData.email,
                    company: devData.company,
                    title: devData.title,
                    created_at: devData.created_at,
                }

                setDeveloper(developer)
                setAccessToken(access_token)
                localStorage.setItem('openbank_token', access_token)
                localStorage.setItem('openbank_developer', JSON.stringify(developer))

                toastUtils.authenticationSuccess(developer.name)
            } else {
                throw new Error(response.message || 'Login failed')
            }
        } catch (error: any) {
            console.error('Login error:', error)
            toast.error(error.message || 'Login failed. Please try again.')
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const logout = () => {
        setDeveloper(null)
        setAccessToken(null)
        setProjects([])
        localStorage.removeItem('openbank_token')
        localStorage.removeItem('openbank_developer')
        toastUtils.logoutSuccess()
    }

    const refreshToken = async () => {
        if (!accessToken) return

        try {
            // Validate current token
            const response = await apiClient.validateToken(accessToken)
            if (response.status !== 'success') {
                throw new Error('Token validation failed')
            }
        } catch (error) {
            console.error('Token refresh failed:', error)
            toast.error('Session expired. Please log in again.')
            logout()
        }
    }

    const createProject = async (data: CreateProjectRequest): Promise<Project> => {
        if (!developer || !accessToken) {
            throw new Error('Not authenticated')
        }

        setError(null)
        setIsLoading(true)
        try {
            const response = await apiClient.createProject(developer.id, data, accessToken)

            if (response.status === 'success' && response.data) {
                const project: Project = {
                    id: response.data.id,
                    name: response.data.name,
                    description: response.data.description,
                    environment: response.data.environment,
                    client_id: response.data.client_id,
                    redirect_uris: response.data.redirect_uris,
                    scopes: response.data.scopes,
                    is_active: response.data.is_active,
                    created_at: response.data.created_at,
                }

                setProjects(prev => [...prev, project])
                toastUtils.projectCreated(project.name)
                return project
            } else {
                throw new Error(response.message || 'Failed to create project')
            }
        } catch (error: any) {
            console.error('Create project error:', error)
            const errorMessage = error.message || 'Failed to create project'
            setError(errorMessage)
            toast.error(`❌ ${errorMessage}`)
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const updateProject = async (projectId: string, data: any): Promise<Project> => {
        if (!developer || !accessToken) {
            throw new Error('Not authenticated')
        }

        setError(null)
        setIsLoading(true)
        try {
            const response = await apiClient.updateProject(developer.id, projectId, data, accessToken)

            if (response.status === 'success' && response.data) {
                const updatedProject: Project = {
                    id: response.data.id,
                    name: response.data.name,
                    description: response.data.description,
                    environment: response.data.environment,
                    client_id: response.data.client_id,
                    redirect_uris: response.data.redirect_uris,
                    scopes: response.data.scopes,
                    is_active: response.data.is_active,
                    created_at: response.data.created_at,
                }

                setProjects(prev => prev.map(p => p.id === projectId ? updatedProject : p))
                toast.success(`✅ Project "${updatedProject.name}" updated successfully!`)
                return updatedProject
            } else {
                throw new Error(response.message || 'Failed to update project')
            }
        } catch (error: any) {
            console.error('Update project error:', error)
            const errorMessage = error.message || 'Failed to update project'
            setError(errorMessage)
            toast.error(`❌ ${errorMessage}`)
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const deleteProject = async (projectId: string): Promise<void> => {
        if (!developer || !accessToken) {
            throw new Error('Not authenticated')
        }

        setError(null)
        setIsLoading(true)
        try {
            const response = await apiClient.deleteProject(developer.id, projectId, accessToken)

            if (response.status === 'success') {
                const deletedProject = projects.find(p => p.id === projectId)
                setProjects(prev => prev.filter(p => p.id !== projectId))
                toastUtils.projectDeleted(deletedProject?.name || 'Unknown')
            } else {
                throw new Error(response.message || 'Failed to delete project')
            }
        } catch (error: any) {
            console.error('Delete project error:', error)
            const errorMessage = error.message || 'Failed to delete project'
            setError(errorMessage)
            toast.error(`❌ ${errorMessage}`)
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const loadProjects = async (): Promise<void> => {
        if (!developer || !accessToken) {
            return
        }

        // TODO: Backend doesn't have GET /developers/:id/projects endpoint yet
        // setIsProjectsLoading(true)
        // setError(null)
        // For now, just set empty projects array
        setProjects([])
        setIsProjectsLoading(false)
        console.log('Load projects: Backend endpoint not implemented yet')
    }

    const generateToken = async (projectId: string, scopes?: string[]): Promise<any> => {
        if (!developer || !accessToken) {
            throw new Error('Not authenticated')
        }

        const project = projects.find(p => p.id === projectId)
        if (!project) {
            throw new Error('Project not found')
        }

        setError(null)
        setIsLoading(true)
        try {
            const [clientId, clientSecret] = project.client_id.split(':')
            const tokenData = {
                grant_type: 'client_credentials',
                client_id: clientId,
                client_secret: clientSecret,
                scope: scopes?.join(' ') || project.scopes.join(' ')
            }

            const response = await apiClient.generateApiToken(tokenData)

            if (response.status === 'success' && response.data) {
                toastUtils.tokenGenerated()
                return response.data
            } else {
                throw new Error(response.message || 'Failed to generate token')
            }
        } catch (error: any) {
            console.error('Generate token error:', error)
            const errorMessage = error.message || 'Failed to generate token'
            setError(errorMessage)
            toast.error(`❌ ${errorMessage}`)
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const refreshProjectToken = async (projectId: string, jti: string): Promise<any> => {
        if (!developer || !accessToken) {
            throw new Error('Not authenticated')
        }

        const project = projects.find(p => p.id === projectId)
        if (!project) {
            throw new Error('Project not found')
        }

        setError(null)
        setIsLoading(true)
        try {
            const [clientId, clientSecret] = project.client_id.split(':')
            const refreshData = {
                client_id: clientId,
                client_secret: clientSecret,
                jti: jti
            }

            const response = await apiClient.refreshApiToken(refreshData)

            if (response.status === 'success' && response.data) {
                toast.success(`🔄 Token refreshed successfully!`)
                return response.data
            } else {
                throw new Error(response.message || 'Failed to refresh token')
            }
        } catch (error: any) {
            console.error('Refresh token error:', error)
            const errorMessage = error.message || 'Failed to refresh token'
            setError(errorMessage)
            toast.error(`❌ ${errorMessage}`)
            throw error
        } finally {
            setIsLoading(false)
        }
    }

    const loadAvailableScopes = async (): Promise<void> => {
        // TODO: Backend doesn't have GET /scopes endpoint yet
        // For now, set some mock scopes
        setAvailableScopes({
            identity: {
                'identity:read': {
                    description: 'Read identity information',
                    permissions: ['read']
                },
                'identity:verify': {
                    description: 'Verify user identity',
                    permissions: ['read', 'write']
                }
            },
            payments: {
                'payments:read': {
                    description: 'Read payment information',
                    permissions: ['read']
                },
                'payments:create': {
                    description: 'Create payments',
                    permissions: ['write']
                }
            },
            transactions: {
                'transactions:read': {
                    description: 'Read transaction history',
                    permissions: ['read']
                }
            }
        })
        console.log('Load scopes: Using mock data, backend endpoint not implemented yet')
    }

    useEffect(() => {
        const savedDeveloper = localStorage.getItem('openbank_developer')
        if (savedDeveloper && accessToken) {
            const developer = JSON.parse(savedDeveloper)
            setDeveloper(developer)

            // TODO: Token validation is causing JWT audience issues
            // For now, skip validation and trust the stored token
            console.log('Skipping token validation to prevent JWT audience errors')
        }
    }, [accessToken])

    const value: AuthContextType = {
        developer,
        projects,
        accessToken,
        availableScopes,
        login,
        logout,
        refreshToken,
        createProject,
        updateProject,
        deleteProject,
        loadProjects,
        generateToken,
        refreshProjectToken,
        loadAvailableScopes,
        isLoading,
        isProjectsLoading,
        error,
    }

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}