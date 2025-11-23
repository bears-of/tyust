<template>
  <div class="profile-container">
    <el-row :gutter="24">
      <el-col :span="12">
        <el-card shadow="never" class="profile-card">
          <template #header>
            <div class="card-header">
              <span class="header-title">账户信息</span>
            </div>
          </template>
          
          <el-descriptions :column="1" border size="large">
            <el-descriptions-item label="用户名">
              <span class="profile-value">{{ username }}</span>
            </el-descriptions-item>
            <el-descriptions-item label="最后登录">
              <span class="profile-value">{{ lastLogin }}</span>
            </el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card shadow="never" class="form-card">
          <template #header>
            <div class="card-header">
              <span class="header-title">修改密码</span>
            </div>
          </template>
          
          <el-form
            ref="passwordFormRef"
            :model="passwordForm"
            :rules="passwordRules"
            label-width="120px"
            class="password-form"
          >
            <el-form-item label="当前密码" prop="oldPassword">
              <el-input
                v-model="passwordForm.oldPassword"
                type="password"
                placeholder="请输入当前密码"
                size="large"
                show-password
              />
            </el-form-item>
            
            <el-form-item label="新密码" prop="newPassword">
              <el-input
                v-model="passwordForm.newPassword"
                type="password"
                placeholder="请输入新密码"
                size="large"
                show-password
              />
            </el-form-item>
            
            <el-form-item label="确认新密码" prop="confirmPassword">
              <el-input
                v-model="passwordForm.confirmPassword"
                type="password"
                placeholder="请再次输入新密码"
                size="large"
                show-password
              />
            </el-form-item>
            
            <el-form-item>
              <el-button
                type="primary"
                :loading="submitting"
                @click="handleUpdatePassword"
                size="large"
                style="width: 120px;"
              >
                更新密码
              </el-button>
              <el-button @click="resetPasswordForm" size="large" style="margin-left: 16px;">
                重置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
        
        <el-card shadow="never" class="form-card" style="margin-top: 24px;">
          <template #header>
            <div class="card-header">
              <span class="header-title">修改用户名</span>
            </div>
          </template>
          
          <el-form
            ref="usernameFormRef"
            :model="usernameForm"
            :rules="usernameRules"
            label-width="120px"
            class="username-form"
          >
            <el-form-item label="新用户名" prop="newUsername">
              <el-input
                v-model="usernameForm.newUsername"
                placeholder="请输入新用户名"
                size="large"
              />
            </el-form-item>
            
            <el-form-item label="当前密码" prop="password">
              <el-input
                v-model="usernameForm.password"
                type="password"
                placeholder="请输入当前密码以确认身份"
                size="large"
                show-password
              />
            </el-form-item>
            
            <el-form-item>
              <el-button
                type="primary"
                :loading="submitting"
                @click="handleUpdateUsername"
                size="large"
                style="width: 120px;"
              >
                更新用户名
              </el-button>
              <el-button @click="resetUsernameForm" size="large" style="margin-left: 16px;">
                重置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { updateAdminPassword, updateAdminUsername } from '@/api/admin'

const passwordFormRef = ref<FormInstance>()
const usernameFormRef = ref<FormInstance>()
const submitting = ref(false)
const username = ref('admin')
const lastLogin = ref('2023-01-01 12:00:00')

const passwordForm = ref({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const usernameForm = ref({
  newUsername: '',
  password: ''
})

const passwordRules: FormRules = {
  oldPassword: [
    { required: true, message: '请输入当前密码', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度至少6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    { 
      validator: (rule: any, value: string, callback: any) => {
        if (value !== passwordForm.value.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      }, 
      trigger: 'blur' 
    }
  ]
}

const usernameRules: FormRules = {
  newUsername: [
    { required: true, message: '请输入新用户名', trigger: 'blur' },
    { min: 3, message: '用户名长度至少3位', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入当前密码', trigger: 'blur' }
  ]
}

const handleUpdatePassword = async () => {
  if (!passwordFormRef.value) return
  
  await passwordFormRef.value.validate(async (valid) => {
    if (valid) {
      submitting.value = true
      try {
        const res: any = await updateAdminPassword({
          old_password: passwordForm.value.oldPassword,
          new_password: passwordForm.value.newPassword
        })
        if (res.code === 0) {
          ElMessage.success('密码更新成功')
          resetPasswordForm()
        } else {
          ElMessage.error(res.message || '密码更新失败')
        }
      } catch (error) {
        ElMessage.error('密码更新失败')
      } finally {
        submitting.value = false
      }
    }
  })
}

const handleUpdateUsername = async () => {
  if (!usernameFormRef.value) return
  
  await usernameFormRef.value.validate(async (valid) => {
    if (valid) {
      submitting.value = true
      try {
        const res: any = await updateAdminUsername({
          new_username: usernameForm.value.newUsername,
          password: usernameForm.value.password
        })
        if (res.code === 0) {
          ElMessage.success('用户名更新成功，请重新登录')
          // 更新本地存储的用户名
          localStorage.setItem('admin_username', usernameForm.value.newUsername)
          // 1秒后跳转到登录页面
          setTimeout(() => {
            localStorage.removeItem('admin_token')
            window.location.href = '/#/login'
          }, 1000)
        } else {
          ElMessage.error(res.message || '用户名更新失败')
        }
      } catch (error) {
        ElMessage.error('用户名更新失败')
      } finally {
        submitting.value = false
      }
    }
  })
}

const resetPasswordForm = () => {
  passwordFormRef.value?.resetFields()
}

const resetUsernameForm = () => {
  usernameFormRef.value?.resetFields()
}

onMounted(() => {
  // 获取当前用户信息
  const storedUsername = localStorage.getItem('admin_username')
  if (storedUsername) {
    username.value = storedUsername
  }
})
</script>

<style scoped>
.profile-container {
  padding: 24px;
  height: calc(100vh - 64px - 48px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.profile-card, .form-card {
  height: 100%;
}

.profile-value {
  font-weight: 500;
  color: #303133;
}

.password-form, .username-form {
  margin-top: 24px;
}
</style>