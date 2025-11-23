<template>
  <el-container class="layout-container">
    <el-aside width="220px" class="sidebar">
      <div class="logo">
        <h2>TYUST 管理后台</h2>
      </div>
      <el-menu
        :default-active="activeMenu"
        router
        background-color="#304156"
        text-color="#bfcbd9"
        active-text-color="#409EFF"
        class="sidebar-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><DataAnalysis /></el-icon>
          <span>仪表盘</span>
        </el-menu-item>
        <el-menu-item index="/students">
          <el-icon><User /></el-icon>
          <span>学生管理</span>
        </el-menu-item>
        <el-menu-item index="/semester">
          <el-icon><Calendar /></el-icon>
          <span>学期设置</span>
        </el-menu-item>
        <el-menu-item index="/profile">
          <el-icon><User /></el-icon>
          <span>账户管理</span>
        </el-menu-item>
      </el-menu>
    </el-aside>
    
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <span class="breadcrumb">{{ currentPageTitle }}</span>
        </div>
        <div class="header-right">
          <span class="username">欢迎，{{ adminStore.username }}</span>
          <el-button type="danger" @click="handleLogout">
            退出登录
          </el-button>
        </div>
      </el-header>
      
      <el-main class="main-content">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessageBox, ElMessage } from 'element-plus'
import { DataAnalysis, User, Calendar } from '@element-plus/icons-vue'
import { useAdminStore } from '@/stores/admin'

const router = useRouter()
const route = useRoute()
const adminStore = useAdminStore()

const activeMenu = computed(() => route.path)
const currentPageTitle = computed(() => route.meta.title as string || '仪表盘')

const handleLogout = () => {
  ElMessageBox.confirm('确定要退出登录吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    adminStore.logout()
    ElMessage.success('已退出登录')
    router.push('/login')
  })
}
</script>

<style scoped>
.layout-container {
  height: 100vh;
  min-width: 1200px; /* 最小宽度确保桌面端体验 */
}

.sidebar {
  background-color: #304156;
  overflow-x: hidden;
  box-shadow: 2px 0 8px rgba(0, 21, 41, 0.1);
}

.sidebar-menu {
  border-right: none;
}

.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #2b3a4b;
  border-bottom: 1px solid #435061;
}

.logo h2 {
  color: #fff;
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.header {
  background-color: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 30px;
  box-shadow: 0 1px 4px rgba(0, 21, 41, 0.08);
  height: 64px;
}

.header-left .breadcrumb {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.username {
  color: #606266;
  font-size: 14px;
}

.main-content {
  background-color: #f0f2f5;
  padding: 24px;
  height: calc(100vh - 64px);
  overflow-y: auto;
}
</style>
