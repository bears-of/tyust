<template>
  <div class="students-container">
    <el-card shadow="never" class="students-card">
      <template #header>
        <div class="card-header">
          <span class="header-title">学生列表</span>
          <div class="header-actions">
            <el-input
              v-model="searchKeyword"
              placeholder="搜索学号或姓名"
              style="width: 200px; margin-right: 16px;"
              clearable
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
            </el-input>
            <el-button type="primary" @click="loadStudents" :loading="loading">
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
          </div>
        </div>
      </template>
      
      <el-table
        :data="filteredStudents"
        v-loading="loading"
        stripe
        style="width: 100%"
        :border="true"
        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
        height="calc(100vh - 280px)"
      >
        <el-table-column type="index" label="#" width="80" align="center" />
        <el-table-column prop="studentId" label="学号" width="180" sortable />
        <el-table-column prop="name" label="姓名" width="150" sortable />
        <el-table-column prop="class" label="班级" min-width="200" />
        <el-table-column prop="token" label="Token" min-width="300" />
        <el-table-column label="操作" width="180" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="viewDetail(row)">
              查看详情
            </el-button>
            <el-button type="info" size="small" style="margin-left: 8px;">
              编辑
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination-container" v-if="students.length > 0">
        <el-pagination
          background
          layout="total, sizes, prev, pager, next, jumper"
          :total="students.length"
          :page-sizes="[10, 20, 50, 100]"
          v-model:page-size="pageSize"
          v-model:current-page="currentPage"
        />
      </div>
    </el-card>
    
    <!-- 详情对话框 -->
    <el-dialog
      v-model="detailDialogVisible"
      title="学生详情"
      width="600px"
      class="student-detail-dialog"
    >
      <el-descriptions :column="1" border v-if="currentStudent" size="large">
        <el-descriptions-item label="学号">{{ currentStudent.studentId }}</el-descriptions-item>
        <el-descriptions-item label="姓名">{{ currentStudent.name }}</el-descriptions-item>
        <el-descriptions-item label="班级">{{ currentStudent.class }}</el-descriptions-item>
        <el-descriptions-item label="Token">
          <el-input type="textarea" :value="currentStudent.token" readonly />
        </el-descriptions-item>
      </el-descriptions>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="detailDialogVisible = false">关闭</el-button>
          <el-button type="primary">编辑信息</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Refresh, Search } from '@element-plus/icons-vue'
import { getStudentList } from '@/api/admin'
import type { Student } from '@/api/admin'

const students = ref<Student[]>([])
const loading = ref(false)
const detailDialogVisible = ref(false)
const currentStudent = ref<Student | null>(null)
const searchKeyword = ref('')
const pageSize = ref(10)
const currentPage = ref(1)

const filteredStudents = computed(() => {
  if (!searchKeyword.value) {
    return students.value
  }
  
  const keyword = searchKeyword.value.toLowerCase()
  return students.value.filter(student => 
    student.studentId.toLowerCase().includes(keyword) ||
    student.name.toLowerCase().includes(keyword)
  )
})

const loadStudents = async () => {
  loading.value = true
  try {
    const res: any = await getStudentList()
    if (res.code === 0 && res.data) {
      students.value = res.data
      ElMessage.success('加载成功')
    }
  } catch (error) {
    ElMessage.error('加载失败')
  } finally {
    loading.value = false
  }
}

const formatDate = (dateStr: string) => {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN')
}

const viewDetail = (student: Student) => {
  currentStudent.value = student
  detailDialogVisible.value = true
}

onMounted(() => {
  loadStudents()
})
</script>

<style scoped>
.students-container {
  padding: 24px;
  height: calc(100vh - 64px - 48px);
}

.students-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
}

.header-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.header-actions {
  display: flex;
  align-items: center;
}

.pagination-container {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
  padding: 16px 0;
}

.student-detail-dialog :deep(.el-dialog__body) {
  padding: 24px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 0 24px 24px;
}
</style>
