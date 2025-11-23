<template>
  <div class="semester-container">
    <el-row :gutter="24">
      <el-col :span="12">
        <el-card shadow="never" class="config-card">
          <template #header>
            <div class="card-header">
              <span class="header-title">当前学期配置</span>
              <el-button type="primary" link @click="loadConfig">
                <el-icon><Refresh /></el-icon>
                刷新
              </el-button>
            </div>
          </template>
          
          <el-descriptions :column="1" border v-if="currentConfig" size="large">
            <el-descriptions-item label="学期名称">
              <span class="config-value">{{ currentConfig.semester_name || '未设置' }}</span>
            </el-descriptions-item>
            <el-descriptions-item label="开学日期">
              <span class="config-value">{{ currentConfig.semester_start_date || '未设置' }}</span>
            </el-descriptions-item>
            <el-descriptions-item label="配置状态">
              <el-tag type="success" v-if="currentConfig.semester_name && currentConfig.semester_start_date">已配置</el-tag>
              <el-tag type="warning" v-else>未完整配置</el-tag>
            </el-descriptions-item>
          </el-descriptions>
          
          <el-empty v-else description="暂无学期配置" />
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card shadow="never" class="form-card">
          <template #header>
            <div class="card-header">
              <span class="header-title">设置学期信息</span>
              <el-button type="info" link @click="resetForm">
                <el-icon><RefreshLeft /></el-icon>
                重置
              </el-button>
            </div>
          </template>
          
          <el-form
            ref="formRef"
            :model="form"
            :rules="rules"
            label-width="120px"
            class="semester-form"
          >
            <el-form-item label="学期名称" prop="semester_name">
              <el-input
                v-model="form.semester_name"
                placeholder="例如: 2024-2025学年第一学期"
                size="large"
              />
            </el-form-item>
            
            <el-form-item label="开学日期" prop="start_date">
              <el-date-picker
                v-model="form.start_date"
                type="date"
                placeholder="选择开学日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                style="width: 100%"
                size="large"
              />
            </el-form-item>
            
            <el-form-item>
              <el-button
                type="primary"
                :loading="submitting"
                @click="handleSubmit"
                size="large"
                style="width: 120px;"
              >
                保存设置
              </el-button>
              <el-button @click="resetForm" size="large" style="margin-left: 16px;">
                重置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="24" style="margin-top: 24px;">
      <el-col :span="24">
        <el-card shadow="never">
          <template #header>
            <div class="card-header">
              <span class="header-title">操作说明</span>
            </div>
          </template>
          <div class="instructions">
            <p><strong>学期名称格式建议：</strong>2024-2025学年第一学期</p>
            <p><strong>开学日期：</strong>请选择实际的开学日期</p>
            <p><strong>保存设置：</strong>点击保存按钮后，新的学期配置将立即生效</p>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { Refresh, RefreshLeft } from '@element-plus/icons-vue'
import { getSemesterConfig, setSemesterConfig } from '@/api/admin'
import type { SemesterConfig } from '@/api/admin'

const formRef = ref<FormInstance>()
const currentConfig = ref<SemesterConfig | null>(null)
const submitting = ref(false)

const form = reactive({
  semester_name: '',
  start_date: ''
})

const rules: FormRules = {
  semester_name: [
    { required: true, message: '请输入学期名称', trigger: 'blur' }
  ],
  start_date: [
    { required: true, message: '请选择开学日期', trigger: 'change' }
  ]
}

const loadConfig = async () => {
  try {
    const res: any = await getSemesterConfig()
    if (res.code === 0 && res.data) {
      currentConfig.value = res.data
      form.semester_name = res.data.semester_name
      form.start_date = res.data.semester_start_date
    }
  } catch (error) {
    console.log('暂无配置')
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      submitting.value = true
      try {
        const res: any = await setSemesterConfig(form)
        if (res.code === 0) {
          ElMessage.success('设置成功')
          await loadConfig()
        }
      } catch (error) {
        ElMessage.error('设置失败')
      } finally {
        submitting.value = false
      }
    }
  })
}

const resetForm = () => {
  formRef.value?.resetFields()
}

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
.semester-container {
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

.config-card, .form-card {
  height: 100%;
}

.config-value {
  font-weight: 500;
  color: #303133;
}

.semester-form {
  margin-top: 24px;
}

.instructions {
  padding: 16px 0;
  line-height: 1.8;
}

.instructions p {
  margin: 8px 0;
  color: #606266;
}
</style>
