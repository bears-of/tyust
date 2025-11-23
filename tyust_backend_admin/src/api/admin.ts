import request from '@/utils/request'

// 登录接口
export interface LoginParams {
  username: string
  password: string
}

export interface LoginResponse {
  code: number
  data: {
    token: string
    username: string
  }
  message: string
}

export function login(data: LoginParams) {
  return request({
    url: '/admin/login',
    method: 'post',
    data
  })
}

// 获取学生列表
export interface Student {
  studentId: string
  name: string
  class: string
  token: string
}

export function getStudentList() {
  return request({
    url: '/admin/students',
    method: 'get'
  })
}

// 获取学期配置
export interface SemesterConfig {
  semester_name: string
  semester_start_date: string
}

export function getSemesterConfig() {
  return request({
    url: '/admin/semester',
    method: 'get'
  })
}

// 设置学期配置
export interface SetSemesterParams {
  semester_name: string
  start_date: string
}

export function setSemesterConfig(data: SetSemesterParams) {
  return request({
    url: '/admin/semester',
    method: 'post',
    data
  })
}

// 获取统计信息
export interface Statistics {
  total_students: number
  active_students: number
  total_logins: number
}

export function getStatistics() {
  return request({
    url: '/admin/statistics',
    method: 'get'
  })
}

// 修改管理员密码
export interface UpdatePasswordParams {
  old_password: string
  new_password: string
}

export function updateAdminPassword(data: UpdatePasswordParams) {
  return request({
    url: '/admin/password',
    method: 'post',
    data
  })
}

// 修改管理员用户名
export interface UpdateUsernameParams {
  new_username: string
  password: string
}

export function updateAdminUsername(data: UpdateUsernameParams) {
  return request({
    url: '/admin/username',
    method: 'post',
    data
  })
}