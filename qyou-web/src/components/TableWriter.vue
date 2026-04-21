<template>
    <div class="scroller fullscreen" style="gap: 20px; max-width: 500px; padding: 20px;" v-if="model">
        <div v-for="(ans, index) in model.answers" :key="index" class="card">
            <div class="row">
                <span>Q{{ index + 1 }}.</span>
                <span>{{ ans.question }}</span>
                <div v-if="table.questions[index]?.required">
                    *
                </div>
            </div>

            <div class="row">
                <OnlyoneWriter v-if="'Onlyone' in ans.choice" v-model="ans.choice.Onlyone" />

                <MultipleWriter v-else-if="'Multiple' in ans.choice" v-model="ans.choice.Multiple" />

                <AskWriter
                    v-else-if="'Ask' in ans.choice && table.questions[index] && 'Ask' in table.questions[index].choice"
                    v-model="ans.choice.Ask" :notification="table.questions[index].choice.Ask.notification" />
            </div>
        </div>
        <div class="card" v-if="model.who">
            <div class="col">
                <div class="row">
                    <label style="width: 80px;">姓名</label>
                    <input v-model="model.who.name" placeholder="您的姓名" />
                </div>
                <div class="row">
                    <label style="width: 80px;">id</label>
                    <input v-model="model.who.identity" placeholder="您的id" />
                </div>
            </div>
        </div>

        <div class="button" @click="submit" style="margin-top: 20px; height: 50px; font-size: 20px;">
            提交问卷
        </div>
    </div>
</template>

<script setup lang="ts">
import { type AnswerTable, type Table } from '@/utils/interfaces';; // 假设你把之前的函数导出了
import OnlyoneWriter from './OnlyoneWriter.vue';
import MultipleWriter from './MultipleWriter.vue';
import AskWriter from './AskWriter.vue';
import { ref } from 'vue';

const model = defineModel<AnswerTable>()
const { table } = defineProps<{ table: Table }>()
const emit = defineEmits(['submit']);



const submit = () => {
    // 1. 检查基本信息 (姓名/身份) 是否必填 (可选)
    if (model.value?.who) {
        if (!model.value.who.name?.trim() || !model.value.who.identity?.trim()) {
            alert("请填写您的姓名和身份信息");
            return;
        }
    }

    // 2. 检查题目必填项
    const answers = model.value?.answers || [];

    for (let i = 0; i < table.questions.length; i++) {
        const questionDefine = table.questions[i];
        const userAnswer = answers[i];
        if (!questionDefine) {
            return
        }

        if (!userAnswer) {
            return
        }
        // 如果该题标记为必填
        if (questionDefine.required) {
            let isEmpty = false;

            if ('Onlyone' in userAnswer.choice) {
                // 单选：检查是否有值
                if (userAnswer.choice.Onlyone.answer === null || userAnswer.choice.Onlyone.answer === 255) {
                    isEmpty = true;
                }
            } else if ('Multiple' in userAnswer.choice) {
                // 多选：检查数组是否为空
                if (!userAnswer.choice.Multiple || userAnswer.choice.Multiple.answer.length === 0) {
                    isEmpty = true;
                }
            } else if ('Ask' in userAnswer.choice) {
                // 问答：检查文本是否为空
                if (!userAnswer.choice.Ask.answer.trim()) {
                    isEmpty = true;
                }
            }

            if (isEmpty) {
                alert(`第 ${i + 1} 题是必填项，请填写后再提交。`);
                // 进阶：这里可以做滚动定位到该题目
                return;
            }
        }
    }

    // 3. 所有检查通过，执行提交
    emit('submit');
};
</script>