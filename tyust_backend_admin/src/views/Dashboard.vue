<template>
  <div class="dashboard">
    <el-row :gutter="24">
      <el-col :span="8">
        <el-card class="stat-card" shadow="hover">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #409EFF;">
              <el-icon :size="48"><User /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ statistics.total_students }}</div>
              <div class="stat-label">总学生数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="stat-card" shadow="hover">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #67C23A;">
              <el-icon :size="48"><UserFilled /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ statistics.active_students }}</div>
              <div class="stat-label">活跃学生</div>
            </div>
          </div>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="stat-card" shadow="hover">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #E6A23C;">
              <el-icon :size="48"><Connection /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ statistics.total_logins }}</div>
              <div class="stat-label">总登录次数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="24" style="margin-top: 24px;">
      <el-col :span="24">
        <el-card class="system-info-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <span>系统信息</span>
            </div>
          </template>
          <el-descriptions :column="3" border size="large">
            <el-descriptions-item label="系统名称">太原科技大学教务系统</el-descriptions-item>
            <el-descriptions-item label="版本号">v1.0.0</el-descriptions-item>
            <el-descriptions-item label="后端地址">http://localhost:3000</el-descriptions-item>
            <el-descriptions-item label="数据库">PostgreSQL</el-descriptions-item>
            <el-descriptions-item label="运行环境">开发环境</el-descriptions-item>
            <el-descriptions-item label="技术支持">TYUST 技术团队</el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="24" style="margin-top: 24px;">
      <el-col :span="24">
        <el-card shadow="hover">
          <template #header>
            <div class="card-header">
              <span>快速操作</span>
            </div>
          </template>
          <div class="quick-actions">
            <el-button type="primary" size="large" @click="$router.push('/students')">
              <el-icon><User /></el-icon>
              查看学生列表
            </el-button>
            <el-button type="success" size="large" @click="$router.push('/semester')">
              <el-icon><Calendar /></el-icon>
              设置学期信息
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { User, UserFilled, Connection } from '@element-plus/icons-vue'
import { getStatistics } from '@/api/admin'
import type { Statistics } from '@/api/admin'

const statistics = ref<Statistics>({
  total_students: 0,
  active_students: 0,
  total_logins: 0
})

const loadStatistics = async () => {
  try {
    const res: any = await getStatistics()
    if (res.code === 0 && res.data) {
      statistics.value = res.data
    }
  } catch (error) {
    ElMessage.warning('暂无统计数据')
  }
}

onMounted(() => {
  loadStatistics()
})
</script>

<style scoped>
.dashboard {
  padding: 24px;
  min-height: calc(100vh - 64px - 48px);
}

.stat-card {
  cursor: pointer;
  transition: all 0.3s ease;
  height: 140px;
}

.stat-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15) !important;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 24px;
  height: 100%;
}

.stat-icon {
  width: 90px;
  height: 90px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 36px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 8px;
  line-height: 1.2;
}

.stat-label {
  font-size: 16px;
  color: #909399;
  font-weight: 500;
}

.system-info-card {
  margin-bottom: 24px;
}

.card-header {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.quick-actions {
  display: flex;
  gap: 24px;
  padding: 24px 0;
}

.quick-actions .el-button {
  height: 60px;
  padding: 0 32px;
  font-size: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

@media (max-width: 1400px) {
  .stat-value {
    font-size: 32px;
  }
  
  .stat-icon {
    width: 80px;
    height: 80px;
  }
}
</style>
