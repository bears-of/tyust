import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'

// 创建 axios 实例
const service: AxiosInstance = axios.create({
  baseURL: 'http://localhost:3000/api',
  timeout: 15000
})

// 请求拦截器
service.interceptors.request.use(
  (config) => {
    // 从 localStorage 获取 token
    const token = localStorage.getItem('admin_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    console.error('Request error:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  (response: AxiosResponse) => {
    const res = response.data
    
    // 如果响应成功
    if (res.code === 0 || response.status === 200) {
      return res
    }
    
    // 处理业务错误
    console.error('Business error:', res.message || 'Unknown error')
    return Promise.reject(new Error(res.message || 'Request failed'))
  },
  (error) => {
    console.error('Response error:', error.message)
    
    // 401 未授权
    if (error.response?.status === 401) {
      localStorage.removeItem('admin_token')
      localStorage.removeItem('admin_info')
      window.location.href = '/login'
    }
    
    return Promise.reject(error)
  }
)

export default service
