import { defineStore } from 'pinia'
import { ref } from 'vue'
import { login as loginApi } from '@/api/admin'
import type { LoginParams } from '@/api/admin'

export const useAdminStore = defineStore('admin', () => {
  const token = ref<string>(localStorage.getItem('admin_token') || '')
  const username = ref<string>(localStorage.getItem('admin_username') || '')
  const isLoggedIn = ref<boolean>(!!token.value)

  // 登录
  async function login(loginData: LoginParams) {
    try {
      const res: any = await loginApi(loginData)
      if (res.code === 0 && res.data) {
        token.value = res.data.token
        username.value = res.data.username
        isLoggedIn.value = true
        
        localStorage.setItem('admin_token', res.data.token)
        localStorage.setItem('admin_username', res.data.username)
        
        return true
      }
      return false
    } catch (error) {
      console.error('Login failed:', error)
      return false
    }
  }

  // 登出
  function logout() {
    token.value = ''
    username.value = ''
    isLoggedIn.value = false
    
    localStorage.removeItem('admin_token')
    localStorage.removeItem('admin_username')
  }

  return {
    token,
    username,
    isLoggedIn,
    login,
    logout
  }
})