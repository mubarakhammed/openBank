// API client for OpenBank backend
const API_BASE_URL = '/api' // Proxied to http://127.0.0.1:8080 in vite.config.ts

export interface ApiResponse<T> {
    status: 'success' | 'error'
    message: string
    data?: T
}

export interface LoginRequest {
    email: string
    password: string
}

export interface LoginResponse {
    developer: {
        id: string
        name: string
        email: string
        company?: string
        title?: string
        created_at: string
    }
    access_token: string
    token_type: string
    expires_in: number
}

export interface RegisterDeveloperRequest {
    name: string
    email: string
    password: string
    company?: string
    title?: string
}

export interface DeveloperResponse {
    id: string
    name: string
    email: string
    company?: string
    title?: string
    created_at: string
}

export interface CreateProjectRequest {
    name: string
    description?: string
    environment: 'development' | 'staging' | 'production'
    redirect_uris: string[]
    scopes: string[]
}

export interface ProjectResponse {
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

export interface TokenRequest {
    grant_type: string
    client_id: string
    client_secret: string
    scope?: string
}

export interface TokenResponse {
    access_token: string
    token_type: string
    expires_in: number
    scope?: string
}

export interface MeResponse {
    developer_id: string
    project_id?: string
    scopes: string[]
    expires_at: string
}

export interface UpdateProjectRequest {
    name?: string
    description?: string
    environment?: 'development' | 'staging' | 'production'
    redirect_uris?: string[]
    scopes?: string[]
    is_active?: boolean
}

export interface DeleteProjectResponse {
    message: string
}

export interface GetProjectsResponse {
    projects: ProjectResponse[]
}

export interface RefreshTokenRequest {
    client_id: string
    client_secret: string
    jti: string
}

export interface AvailableScopesResponse {
    scopes: {
        [category: string]: {
            [scope: string]: {
                description: string
                permissions: string[]
            }
        }
    }
}

class ApiClient {
    private baseUrl: string

    constructor(baseUrl: string = API_BASE_URL) {
        this.baseUrl = baseUrl
    }

    private async request<T>(
        endpoint: string,
        options: RequestInit = {}
    ): Promise<ApiResponse<T>> {
        const url = `${this.baseUrl}${endpoint}`

        const config: RequestInit = {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers,
            },
            ...options,
        }

        try {
            const response = await fetch(url, config)

            if (!response.ok) {
                const errorData = await response.text()
                throw new Error(`HTTP ${response.status}: ${errorData}`)
            }

            const data = await response.json()
            return data
        } catch (error) {
            console.error('API request failed:', error)
            throw error
        }
    }

    // Auth endpoints
    async login(credentials: LoginRequest): Promise<ApiResponse<LoginResponse>> {
        return this.request<LoginResponse>('/auth/login', {
            method: 'POST',
            body: JSON.stringify(credentials),
        })
    }

    async register(data: RegisterDeveloperRequest): Promise<ApiResponse<DeveloperResponse>> {
        return this.request<DeveloperResponse>('/auth/developers', {
            method: 'POST',
            body: JSON.stringify(data),
        })
    }

    async validateToken(token: string): Promise<ApiResponse<MeResponse>> {
        return this.request<MeResponse>('/auth/me', {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async createProject(
        developerId: string,
        data: CreateProjectRequest,
        token: string
    ): Promise<ApiResponse<ProjectResponse>> {
        return this.request<ProjectResponse>(`/auth/developers/${developerId}/projects`, {
            method: 'POST',
            body: JSON.stringify(data),
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async generateApiToken(data: TokenRequest): Promise<ApiResponse<TokenResponse>> {
        return this.request<TokenResponse>('/auth/token', {
            method: 'POST',
            body: JSON.stringify(data),
        })
    }

    async refreshApiToken(data: RefreshTokenRequest): Promise<ApiResponse<TokenResponse>> {
        return this.request<TokenResponse>('/auth/token/refresh', {
            method: 'POST',
            body: JSON.stringify(data),
        })
    }

    async getProjects(developerId: string, token: string): Promise<ApiResponse<GetProjectsResponse>> {
        return this.request<GetProjectsResponse>(`/auth/developers/${developerId}/projects`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async getProject(
        developerId: string,
        projectId: string,
        token: string
    ): Promise<ApiResponse<ProjectResponse>> {
        return this.request<ProjectResponse>(`/auth/developers/${developerId}/projects/${projectId}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async updateProject(
        developerId: string,
        projectId: string,
        data: UpdateProjectRequest,
        token: string
    ): Promise<ApiResponse<ProjectResponse>> {
        return this.request<ProjectResponse>(`/auth/developers/${developerId}/projects/${projectId}`, {
            method: 'PUT',
            body: JSON.stringify(data),
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async deleteProject(
        developerId: string,
        projectId: string,
        token: string
    ): Promise<ApiResponse<DeleteProjectResponse>> {
        return this.request<DeleteProjectResponse>(`/auth/developers/${developerId}/projects/${projectId}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    }

    async getAvailableScopes(): Promise<ApiResponse<AvailableScopesResponse>> {
        return this.request<AvailableScopesResponse>('/auth/scopes')
    }

    // Health check
    async healthCheck(): Promise<any> {
        return this.request('/health')
    }
}

export const apiClient = new ApiClient()