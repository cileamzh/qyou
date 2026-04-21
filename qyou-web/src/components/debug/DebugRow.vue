<template>
  <tr :class="{ 'row-nested': isObject, 'has-parent': depth > 0 }">
    <td 
      :style="{ paddingLeft: (depth * 20 + 12) + 'px' }" 
      class="key-cell"
    >
      <div v-if="depth > 0" class="indent-line" :style="{ left: (depth * 20 - 4) + 'px' }"></div>
      
      <span class="key-text">{{ k }}</span>
    </td>
    <td class="value-cell">
      <template v-if="isObject">
        <span class="type-badge">{{ Array.isArray(v) ? 'Array' : 'Object' }}</span>
        <span class="item-count">{{ Object.keys(v).length }} items</span>
      </template>
      <template v-else>
        <span :class="['val-text', typeof v]">{{ formatValue(v) }}</span>
      </template>
    </td>
  </tr>
  
  <template v-if="isObject">
    <DebugRow 
      v-for="(subVal, subKey) in v" 
      :key="subKey" 
      :k="subKey" 
      :v="subVal" 
      :depth="depth + 1" 
    />
  </template>
</template>

<script setup lang="ts">
const props = defineProps<{ k: string | number, v: any, depth: number }>();
const isObject = props.v !== null && typeof props.v === 'object';

const formatValue = (val: any) => {
  if (val === null) return 'null';
  if (typeof val === 'string') return `"${val}"`;
  return String(val);
};
</script>

<style scoped>
tr {
    border-bottom: 1px solid #f1f5f9;
    position: relative;
}

.key-cell {
    position: relative; /* 为缩进线定位 */
    width: 45%;
    padding: 10px 8px;
    vertical-align: middle;
}

/* --- 核心：垂直缩进线 --- */
.indent-line {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 1.5px;
    background-color: #e2e8f0; /* 线条颜色 */
    transition: background-color 0.2s;
}

tr:hover .indent-line {
    background-color: #3b82f6; /* 悬浮时高亮对应的缩进线 */
}

.key-text {
    font-family: 'Inter', system-ui, sans-serif;
    font-size: 13px;
    font-weight: 600;
    color: #334155;
}

.value-cell {
    padding: 10px 8px;
    vertical-align: middle;
}

.type-badge {
    font-size: 10px;
    background: #eff6ff;
    color: #3b82f6;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 800;
    margin-right: 6px;
}

.item-count {
    font-size: 10px;
    color: #94a3b8;
}

.val-text {
    font-family: 'Fira Code', 'Cascadia Code', monospace;
    font-size: 12px;
}

/* 不同类型的色彩 */
.val-text.string { color: #059669; }
.val-text.number { color: #d97706; }
.val-text.boolean { color: #2563eb; font-weight: 700; }

.row-nested {
    background: rgba(248, 250, 252, 0.8);
}
</style>