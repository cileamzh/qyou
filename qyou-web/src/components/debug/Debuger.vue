<template>
    <Transition name="panel-slide">
        <div v-if="show" class="debug-panel" :class="{ 'is-collapsed': isCollapsed }">
            <div class="debug-header" @click="isCollapsed = !isCollapsed">
                <div class="row header-inner">
                    <div class="status-group">
                        <div class="status-dot"></div>
                        <span class="status-text">Debuger</span>
                    </div>
                    <div class="action-group">
                        -
                    </div>
                </div>
            </div>

            <div class="debug-body" v-show="!isCollapsed">
                <div class="scroller">
                    <table class="debug-table">
                        <thead>
                            <tr>
                                <th>字段 (Key)</th>
                                <th>数值 (Value)</th>
                            </tr>
                        </thead>
                        <tbody>
                            <DebugRow v-for="(val, key) in data" :key="key" :k="key" :v="val" :depth="0" />
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import DebugRow from './DebugRow.vue';

const show = defineModel<boolean>({ default: false });
const { data } = defineProps<{ data: any }>();
const isCollapsed = ref(true);
</script>

<style scoped>
.debug-panel {
    position: fixed;
    left: 24px;
    bottom: 24px;
    width: 400px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 16px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(0, 0, 0, 0.05);
    z-index: 10000;
    overflow: hidden;
    transition: all 0.5s cubic-bezier(0.19, 1, 0.22, 1);
}

.is-collapsed {
    width: 140px;
    transform: translateY(0);
}

.debug-header {
    padding: 12px 16px;
    background: #f8fafc;
    cursor: pointer;
    border-bottom: 1px solid #e2e8f0;
}

.header-inner {
    justify-content: space-between;
    width: 100%;
}

.status-group {
    display: flex;
    align-items: center;
    gap: 8px;
}

.status-dot {
    width: 8px;
    height: 8px;
    background: #10b981;
    border-radius: 50%;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
}

.status-text {
    font-size: 12px;
    font-weight: 700;
    color: #475569;
}

.action-btn {
    font-size: 11px;
    font-weight: 600;
    color: #94a3b8;
    padding: 4px 8px;
    border-radius: 4px;
}

.action-btn:hover {
    background: #f1f5f9;
    color: #2563eb;
}

.debug-body {
    padding: 8px;
}

.scroller {
    max-height: 450px;
    overflow-y: auto;
}

/* DebugPanel.vue 中的 table 样式 */
.debug-table {
    width: 100%;
    border-collapse: collapse;
    /* 必须为 collapse，确保 tr 之间没有间隙 */
    table-layout: fixed;
    /* 固定布局防止长文本撑开表格 */
}

.debug-table th {
    text-align: left;
    padding: 8px;
    color: #64748b;
    font-size: 11px;
    text-transform: uppercase;
    border-bottom: 2px solid #f1f5f9;
}

/* 动画 */
.panel-slide-enter-active,
.panel-slide-leave-active {
    transition: all 0.4s ease;
}

.panel-slide-enter-from,
.panel-slide-leave-to {
    transform: translateX(-50px) opacity(0);
    opacity: 0;
}
</style>