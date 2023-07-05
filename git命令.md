# git命令

```markdown
ls 罗列目录
cd 跳转到某目录
pwd 显示当前所处工作目录的全路径
mkdir xxx 创建一个目录
git --version 获取git版本号
git remote add origin 网址名
git clone 获取远程仓库
git push -u origin GitHub仓库上分支名 推送代码
git pull origin GitHub仓库上分支名 拉代码
git config --global user.name "Runoob"  设置于本项目的名字
git config --global user.email admin@runoob.com 设置于本项目的邮件地址
git branch 查看分支
git branch -a 同时显示远程仓库和本地仓库的分支信息
git log 查看历史提交记录。
    git log命令可以显示当前分支所有提交过的版本信息，不包括已经被删除的 commit 记录和reset的操作。
git reflog
    git reflog命令可以查看所有分支的所有操作记录信息（包括已经被删除的 commit 记录和 reset 的操作）。
    总结：git log 和 git reflog的最大区别是能不能查询到被删除的 commit 记录和 reset 的操作记录，log     不能，而reflog可以；所以以后要买后悔药就去找reflog。
git log --pretty=oneline   git reflog --pretty=oneline 只显示版本号和提交信息
git commit -help 命令可以查看 commit 的命令选项
git checkout 分支名 切换分支
git checkout 切换回上一个分支
git checkout -b branchname 命令来创建新分支并立即切换到该分支下，从而在该分支中操作
git branch -d branchname 删除分支
git fetch 命令将提交、文件和引用从远程存储库下载到本地存储库中
git diff 命令比较文件的不同，即比较文件在暂存区和工作区的差异。
git init 初始化仓库
touch xxx.xx 创建一个指定格式的文件
git status 查看仓库状态
git add 文件名，文件名  文件提交到暂存区
git add . 提交目前所有未被管理的文件提交到暂存区
git commit 	文件名，文件名 -m '简单版本备注'
git commit -m 'xxx' 默认未被处理的所有文件
git commit  保存仓库历史记录（执行后进入编辑模石）
    保存并退出：
     （1）按 Esc 键退出编辑模式，英文模式下输入 :wq ，然后回车(write and quit)。
     （2）按 Esc 键退出编辑模式，大写英文模式下输入 ZZ ，然后回车。
    不保存退出：
     （1）按 Esc 键退出编辑模式，英文模式下输入 :q! ，然后回车。
     （2）按 Esc 键退出编辑模式，英文模式下输入 :qa! ，然后回车。
git log 查看提交日志
git log --preyyy=short 只显示提交信息第一行
git log 文件名/目录名 只显示与该文件/目录相关的日志
git log -p 显示文件的改动
git diff 查看更改差别 详细自己搜
git diff HEAD  HEAD指向当前分支最新一次提交的指针 
                   一般用在 git commit 前确认两个提交的差别
git merge --no-ff 分支名 （一般用于master和并已成熟的分支）
```

