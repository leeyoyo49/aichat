# AIChat LaTeX 文件編譯說明

## 文件說明

已為您創建了一份專業的 LaTeX 格式 AIChat 功能介紹文件:
- **檔案名稱:** `AICHAT_功能介紹.tex`
- **頁數:** 約 20-25 頁
- **語言:** 繁體中文
- **格式:** A4, 12pt

## 文件特色

### 📚 內容結構
- 完整的專案概述
- 詳細的功能說明(11 個核心功能)
- 實際應用場景
- 進階配置與優化
- 安全性考量
- 疑難排解

### 🎨 視覺設計
- 專業的標題和頁眉頁腳
- 自訂彩色方框(功能、命令、提示)
- 語法高亮的程式碼區塊
- 表格和圖表
- TikZ 繪製的 RAG 架構圖

### 📝 LaTeX 套件使用
- `ctex` - 中文支援
- `listings` - 程式碼高亮
- `tcolorbox` - 彩色方框
- `tikz` - 繪圖
- `hyperref` - 超連結
- `booktabs` - 專業表格
- `fontawesome5` - 圖示

## 編譯方式

### 方法 1: 使用 Makefile (推薦)

```bash
# 檢查環境
make -f Makefile.latex check

# 編譯 PDF
make -f Makefile.latex

# 編譯並開啟
make -f Makefile.latex show

# 清理輔助檔案
make -f Makefile.latex clean

# 顯示所有指令
make -f Makefile.latex help
```

### 方法 2: 直接使用 XeLaTeX

```bash
# 第一次編譯
xelatex AICHAT_功能介紹.tex

# 第二次編譯(生成目錄)
xelatex AICHAT_功能介紹.tex

# 開啟 PDF
open AICHAT_功能介紹.pdf  # macOS
xdg-open AICHAT_功能介紹.pdf  # Linux
```

### 方法 3: 使用 latexmk

```bash
latexmk -xelatex AICHAT_功能介紹.tex
```

## 安裝編譯環境

### macOS

```bash
# 使用 Homebrew 安裝 MacTeX
brew install --cask mactex

# 或安裝精簡版
brew install --cask basictex

# 安裝額外套件
sudo tlmgr update --self
sudo tlmgr install ctex tcolorbox tikz fontawesome5
```

### Ubuntu/Debian

```bash
# 安裝 TeX Live
sudo apt-get update
sudo apt-get install texlive-full texlive-xetex

# 或安裝精簡版
sudo apt-get install texlive-latex-base texlive-xetex

# 安裝額外套件
sudo apt-get install texlive-lang-chinese
```

### Windows

1. 下載安裝 [TeX Live](https://www.tug.org/texlive/) 或 [MiKTeX](https://miktex.org/)
2. 安裝過程中選擇完整安裝
3. 使用命令提示字元或 PowerShell 編譯

### Arch Linux

```bash
sudo pacman -S texlive-core texlive-langchinese
```

## 編譯問題排解

### 問題 1: 找不到 xelatex

**解決方案:**
```bash
# 檢查是否已安裝
which xelatex

# 如果未安裝,請安裝 TeX Live
```

### 問題 2: 缺少套件

**錯誤訊息:**
```
! LaTeX Error: File `ctex.sty' not found.
```

**解決方案:**
```bash
# 使用 tlmgr 安裝缺少的套件
sudo tlmgr install ctex

# 或安裝所有套件
sudo tlmgr install scheme-full
```

### 問題 3: 中文顯示問題

**解決方案:**
```bash
# 確保系統有中文字型
fc-list :lang=zh

# macOS 通常已內建
# Linux 可能需要安裝
sudo apt-get install fonts-noto-cjk
```

### 問題 4: TikZ 圖表錯誤

**解決方案:**
```bash
# 安裝 TikZ 相關套件
sudo tlmgr install pgf tikz
```

## Makefile 指令說明

```bash
# 基本編譯
make -f Makefile.latex          # 編譯 PDF (編譯兩次以生成目錄)
make -f Makefile.latex quick    # 快速編譯(單次)
make -f Makefile.latex full     # 完整編譯(含參考文獻)

# 檢視與清理
make -f Makefile.latex view     # 開啟 PDF
make -f Makefile.latex show     # 編譯並開啟
make -f Makefile.latex clean    # 清理輔助檔案
make -f Makefile.latex distclean# 完全清理(含 PDF)

# 工具指令
make -f Makefile.latex check    # 檢查編譯環境
make -f Makefile.latex help     # 顯示幫助
make -f Makefile.latex watch    # 監控並自動編譯
```

## 自訂修改

### 修改顏色

在 LaTeX 文件中找到顏色定義:

```latex
\definecolor{titlecolor}{RGB}{0,102,204}
\definecolor{sectioncolor}{RGB}{51,102,153}
\definecolor{highlightcolor}{RGB}{255,204,0}
```

修改 RGB 值即可。

### 修改字體大小

修改 `\documentclass` 行:

```latex
\documentclass[12pt,a4paper]{article}  % 改為 10pt, 11pt, 或 12pt
```

### 修改頁面邊距

修改 `geometry` 設定:

```latex
\geometry{left=2.5cm,right=2.5cm,top=3cm,bottom=3cm}
```

### 新增章節

直接在文件中加入:

```latex
\section{新章節標題}
內容...

\subsection{小節標題}
內容...
```

## 輸出檔案

編譯後會產生以下檔案:

- `AICHAT_功能介紹.pdf` - 最終 PDF 檔案 ✅
- `AICHAT_功能介紹.aux` - 輔助檔案
- `AICHAT_功能介紹.log` - 編譯日誌
- `AICHAT_功能介紹.out` - 超連結資訊
- `AICHAT_功能介紹.toc` - 目錄資訊

使用 `make clean` 可清理輔助檔案,只保留 PDF。

## 線上編譯

如果不想安裝 LaTeX,可以使用線上服務:

1. **Overleaf** - https://www.overleaf.com/
   - 上傳 `.tex` 檔案
   - 線上編譯和預覽
   - 免費且功能強大

2. **ShareLaTeX** - https://www.sharelatex.com/
   - 類似 Overleaf
   - 支援協作編輯

### 使用步驟:
1. 註冊帳號
2. 建立新專案
3. 上傳 `AICHAT_功能介紹.tex`
4. 點選 "Recompile"
5. 下載 PDF

## 文件結構說明

```
1. 標題頁
   - 專案名稱
   - 版本資訊
   - 授權資訊

2. 摘要
   - 專案簡介

3. 目錄
   - 自動生成

4. 專案概述 (Section 1)
   - 核心特性
   - 技術架構

5. 核心功能詳解 (Section 2)
   - 多 LLM 提供商支援
   - 三種工作模式
   - Shell 助手
   - 多形式輸入
   - 角色系統
   - 會話系統
   - RAG
   - Function Calling
   - AI Agents
   - 巨集系統

6. 進階功能 (Section 3)
   - 自訂主題
   - 自訂提示符

7. 實際應用場景 (Section 4)
   - 4 個實用範例

8. 效能與優化 (Section 5)
   - 模型選擇
   - Token 優化
   - 本地部署

9. 安全性考量 (Section 6)
   - API Key 管理
   - 敏感資料保護
   - 工具權限

10. 安裝與部署 (Section 7)
    - 安裝方式
    - 配置結構

11. 疑難排解 (Section 8)
    - 常見問題

12. 社群與貢獻 (Section 9)
    - 資源連結
    - 貢獻指南

13. 總結 (Section 10)
    - 功能總結
    - 版本資訊
```

## 效能建議

### 加速編譯

```bash
# 使用 latexmk 的持續編譯模式
latexmk -xelatex -pvc AICHAT_功能介紹.tex

# 使用快速編譯(跳過第二次編譯)
make -f Makefile.latex quick
```

### 減少檔案大小

如果 PDF 太大,可以壓縮:

```bash
# macOS/Linux
gs -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/ebook \
   -dNOPAUSE -dQUIET -dBATCH \
   -sOutputFile=AICHAT_功能介紹_compressed.pdf \
   AICHAT_功能介紹.pdf
```

## 匯出其他格式

### 轉換為 HTML

```bash
# 使用 pandoc
pandoc AICHAT_功能介紹.tex -o AICHAT_功能介紹.html
```

### 轉換為 Word

```bash
pandoc AICHAT_功能介紹.tex -o AICHAT_功能介紹.docx
```

## 列印建議

- **紙張:** A4
- **方向:** 直向
- **雙面列印:** 建議啟用
- **裝訂:** 左側裝訂
- **彩色/黑白:** 建議彩色(有彩色方框和語法高亮)

## 版本控制

建議將 `.tex` 檔案加入 Git,但排除編譯產生的檔案:

```bash
# .gitignore
*.aux
*.log
*.out
*.toc
*.synctex.gz
*.fls
*.fdb_latexmk

# 保留 PDF(可選)
# AICHAT_功能介紹.pdf
```

## 需要幫助?

如果編譯遇到問題:

1. 檢查 `.log` 檔案查看詳細錯誤
2. 確認所有套件都已安裝
3. 嘗試使用 Overleaf 線上編譯
4. 查看 LaTeX 官方文件: https://www.latex-project.org/

---

**提示:** 首次編譯可能需要較長時間,後續編譯會快很多!
