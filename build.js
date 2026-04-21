const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * 跨平台递归复制文件夹或文件
 */
function copyRecursiveSync(src, dest) {
    const exists = fs.existsSync(src);
    const stats = exists && fs.statSync(src);
    const isDirectory = exists && stats.isDirectory();

    if (isDirectory) {
        if (!fs.existsSync(dest)) {
            fs.mkdirSync(dest, { recursive: true });
        }
        fs.readdirSync(src).forEach((childItemName) => {
            copyRecursiveSync(
                path.join(src, childItemName),
                path.join(dest, childItemName)
            );
        });
    } else {
        // 确保目标目录存在
        const destDir = path.dirname(dest);
        if (!fs.existsSync(destDir)) {
            fs.mkdirSync(destDir, { recursive: true });
        }
        fs.copyFileSync(src, dest);
    }
}

/**
 * 封装执行函数
 */
function run(command, cwd = process.cwd()) {
    console.log(`\n> 执行: ${command}`);
    try {
        execSync(command, { stdio: 'inherit', cwd });
    } catch (error) {
        console.error(`\n❌ 命令失败: ${command}`);
        process.exit(1);
    }
}

// --- 路径配置 ---
const rootDir = process.cwd();
const webDir = path.join(rootDir, 'qyou-web');
const staticDir = path.join(rootDir, 'static');
const buildDir = path.join(rootDir, 'build');
const webDistDir = path.join(webDir, 'dist');

console.log('🚀 开始构建流程...');

// 1. 清理并准备目录
[staticDir, buildDir].forEach(dir => {
    if (fs.existsSync(dir)) {
        fs.rmSync(dir, { recursive: true, force: true });
    }
    fs.mkdirSync(dir, { recursive: true });
});

// 2. 后端构建 (Rust)
run('cargo build --release');

// 3. 前端构建 (Node/Vue/React)
run('npm install', webDir);
run('npm run build', webDir);

// 4. 将前端产物移至根目录 static
console.log(`\n📦 正在同步前端产物到 static 目录...`);
if (fs.existsSync(webDistDir)) {
    copyRecursiveSync(webDistDir, staticDir);
} else {
    console.error(`❌ 错误: 未找到前端打包目录 ${webDistDir}`);
    process.exit(1);
}

// 5. 准备最终构建产物 (build 目录)
console.log(`\n打包最终产物到 build 文件夹...`);

// 复制二进制文件
const binaryName = "qyou";
const projectName = path.basename(binaryName);
const exeExtension = process.platform === 'win32' ? '.exe' : '';
const binarySource = path.join(rootDir, 'target', 'release', projectName + exeExtension);

if (fs.existsSync(binarySource)) {
    copyRecursiveSync(binarySource, path.join(buildDir, projectName + exeExtension));
} else {
    console.warn(`⚠️ 警告: 未找到二进制文件 ${binarySource}`);
}

// 复制 static 文件夹到 build 目录
copyRecursiveSync(staticDir, path.join(buildDir, 'static'));

// 6. 创建 config.json
const config = {
    "host": "0.0.0.0:9001",
    "keys": ["qyou"],
    "base": {
        "web_title": "",
        "company": "Qyou"
    }
};
fs.writeFileSync(
    path.join(buildDir, 'config.json'),
    JSON.stringify(config, null, 4),
    'utf8'
);

console.log(`\n✅ 所有任务完成！请检查 ./build 目录`);