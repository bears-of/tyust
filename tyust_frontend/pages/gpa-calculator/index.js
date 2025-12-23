// GPA 计算器页面
Page({
  /**
   * 页面的初始数据
   */
  data: {
    // 课程列表
    courses: [],
    // 显示添加课程弹窗
    showAddModal: false,
    // 当前编辑的课程（新增时为null）
    editingCourse: null,
    editingIndex: -1,
    // 表单数据
    formData: {
      courseName: '',
      credit: '',
      score: ''
    },
    // 计算结果
    totalCredits: 0,
    weightedSum: 0,
    gpa: 0,
    // 计算方式
    calculationMethod: '4.0', // '4.0' 或 '5.0'
    // GPA 标准选择器
    gpaStandards: [
      { name: '4.0 标准', value: '4.0' },
      { name: '5.0 标准', value: '5.0' }
    ],
    gpaStandardIndex: 0
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    // 从缓存中加载数据
    this.loadFromCache();
    // 尝试从成绩页面导入成绩
    this.tryLoadFromScores();
  },

  /**
   * 从缓存加载数据
   */
  loadFromCache() {
    const cachedCourses = wx.getStorageSync('gpaCourses');
    const cachedMethod = wx.getStorageSync('gpaMethod');
    if (cachedCourses && cachedCourses.length > 0) {
      this.setData({
        courses: cachedCourses,
        calculationMethod: cachedMethod || '4.0'
      });
      this.calculateGPA();
    }
  },

  /**
   * 尝试从成绩数据导入
   */
  tryLoadFromScores() {
    // 如果已有课程数据，不自动导入
    if (this.data.courses.length > 0) return;

    const rawScores = wx.getStorageSync('rawScores');
    if (!rawScores || rawScores.length === 0) return;

    // 提取所有成绩
    const importedCourses = [];
    rawScores.forEach(term => {
      if (term.scoreList && term.scoreList.length > 0) {
        term.scoreList.forEach(score => {
          // 只导入有数字成绩的课程
          const numScore = parseFloat(score.score);
          if (!isNaN(numScore)) {
            importedCourses.push({
              courseName: score.name,
              credit: parseFloat(score.credit) || 0,
              score: numScore
            });
          }
        });
      }
    });

    if (importedCourses.length > 0) {
      wx.showModal({
        title: '导入成绩',
        content: `检测到 ${importedCourses.length} 门课程成绩，是否导入？`,
        success: (res) => {
          if (res.confirm) {
            this.setData({
              courses: importedCourses
            });
            this.calculateGPA();
            this.saveToCache();
          }
        }
      });
    }
  },

  /**
   * 保存到缓存
   */
  saveToCache() {
    wx.setStorageSync('gpaCourses', this.data.courses);
    wx.setStorageSync('gpaMethod', this.data.calculationMethod);
  },

  /**
   * 显示添加课程弹窗
   */
  showAddCourseModal() {
    this.setData({
      showAddModal: true,
      editingCourse: null,
      editingIndex: -1,
      formData: {
        courseName: '',
        credit: '',
        score: ''
      }
    });
  },

  /**
   * 显示编辑课程弹窗
   */
  editCourse(e) {
    const index = e.currentTarget.dataset.index;
    const course = this.data.courses[index];
    this.setData({
      showAddModal: true,
      editingCourse: course,
      editingIndex: index,
      formData: {
        courseName: course.courseName,
        credit: course.credit.toString(),
        score: course.score.toString()
      }
    });
  },

  /**
   * 删除课程
   */
  deleteCourse(e) {
    const index = e.currentTarget.dataset.index;
    wx.showModal({
      title: '确认删除',
      content: '确定要删除这门课程吗？',
      success: (res) => {
        if (res.confirm) {
          const courses = this.data.courses;
          courses.splice(index, 1);
          this.setData({
            courses
          });
          this.calculateGPA();
          this.saveToCache();
          wx.showToast({
            title: '已删除',
            icon: 'success'
          });
        }
      }
    });
  },

  /**
   * 关闭弹窗
   */
  closeModal() {
    this.setData({
      showAddModal: false
    });
  },

  /**
   * 表单输入
   */
  onCourseNameInput(e) {
    this.setData({
      'formData.courseName': e.detail.value
    });
  },

  onCreditInput(e) {
    this.setData({
      'formData.credit': e.detail.value
    });
  },

  onScoreInput(e) {
    this.setData({
      'formData.score': e.detail.value
    });
  },

  /**
   * 保存课程
   */
  saveCourse() {
    const { courseName, credit, score } = this.data.formData;

    // 验证
    if (!courseName.trim()) {
      wx.showToast({
        title: '请输入课程名称',
        icon: 'none'
      });
      return;
    }

    const creditNum = parseFloat(credit);
    if (isNaN(creditNum) || creditNum <= 0) {
      wx.showToast({
        title: '请输入有效学分',
        icon: 'none'
      });
      return;
    }

    const scoreNum = parseFloat(score);
    if (isNaN(scoreNum) || scoreNum < 0 || scoreNum > 100) {
      wx.showToast({
        title: '请输入有效成绩(0-100)',
        icon: 'none'
      });
      return;
    }

    const course = {
      courseName: courseName.trim(),
      credit: creditNum,
      score: scoreNum
    };

    const courses = this.data.courses;
    if (this.data.editingIndex >= 0) {
      // 编辑模式
      courses[this.data.editingIndex] = course;
    } else {
      // 新增模式
      courses.push(course);
    }

    this.setData({
      courses,
      showAddModal: false
    });

    this.calculateGPA();
    this.saveToCache();

    wx.showToast({
      title: this.data.editingIndex >= 0 ? '修改成功' : '添加成功',
      icon: 'success'
    });
  },

  /**
   * 切换计算方式
   */
  changeGPAStandard(e) {
    const index = e.detail.value;
    const method = this.data.gpaStandards[index].value;
    this.setData({
      gpaStandardIndex: index,
      calculationMethod: method
    });
    this.calculateGPA();
    this.saveToCache();
  },

  /**
   * 计算 GPA
   */
  calculateGPA() {
    const { courses, calculationMethod } = this.data;

    if (courses.length === 0) {
      this.setData({
        totalCredits: 0,
        weightedSum: 0,
        gpa: 0
      });
      return;
    }

    let totalCredits = 0;
    let weightedSum = 0;

    courses.forEach(course => {
      const credit = course.credit;
      const score = course.score;
      const gradePoint = this.scoreToGradePoint(score, calculationMethod);

      totalCredits += credit;
      weightedSum += credit * gradePoint;
    });

    const gpa = totalCredits > 0 ? (weightedSum / totalCredits).toFixed(2) : 0;

    this.setData({
      totalCredits: totalCredits.toFixed(1),
      weightedSum: weightedSum.toFixed(2),
      gpa
    });
  },

  /**
   * 分数转换为绩点
   */
  scoreToGradePoint(score, method) {
    if (method === '5.0') {
      // 5.0 标准
      if (score >= 90) return 5.0;
      if (score >= 85) return 4.5;
      if (score >= 82) return 4.0;
      if (score >= 78) return 3.5;
      if (score >= 75) return 3.0;
      if (score >= 72) return 2.5;
      if (score >= 68) return 2.0;
      if (score >= 64) return 1.5;
      if (score >= 60) return 1.0;
      return 0;
    } else {
      // 4.0 标准（默认）
      if (score >= 90) return 4.0;
      if (score >= 85) return 3.7;
      if (score >= 82) return 3.3;
      if (score >= 78) return 3.0;
      if (score >= 75) return 2.7;
      if (score >= 72) return 2.3;
      if (score >= 68) return 2.0;
      if (score >= 64) return 1.5;
      if (score >= 60) return 1.0;
      return 0;
    }
  },

  /**
   * 清空所有数据
   */
  clearAll() {
    wx.showModal({
      title: '确认清空',
      content: '确定要清空所有课程数据吗？',
      success: (res) => {
        if (res.confirm) {
          this.setData({
            courses: [],
            totalCredits: 0,
            weightedSum: 0,
            gpa: 0
          });
          this.saveToCache();
          wx.showToast({
            title: '已清空',
            icon: 'success'
          });
        }
      }
    });
  },

  /**
   * 导出数据
   */
  exportData() {
    if (this.data.courses.length === 0) {
      wx.showToast({
        title: '暂无数据可导出',
        icon: 'none'
      });
      return;
    }

    let text = `GPA 计算结果\n\n`;
    text += `计算方式: ${this.data.calculationMethod} 标准\n`;
    text += `总学分: ${this.data.totalCredits}\n`;
    text += `加权总分: ${this.data.weightedSum}\n`;
    text += `GPA: ${this.data.gpa}\n\n`;
    text += `课程明细:\n`;
    text += `序号\t课程名称\t学分\t成绩\t绩点\n`;

    this.data.courses.forEach((course, index) => {
      const gradePoint = this.scoreToGradePoint(course.score, this.data.calculationMethod);
      text += `${index + 1}\t${course.courseName}\t${course.credit}\t${course.score}\t${gradePoint.toFixed(2)}\n`;
    });

    wx.setClipboardData({
      data: text,
      success: () => {
        wx.showToast({
          title: '已复制到剪贴板',
          icon: 'success'
        });
      }
    });
  }
});
