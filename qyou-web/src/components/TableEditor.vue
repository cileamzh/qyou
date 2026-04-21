<template>
    <div id="table-editor" v-if="model">
        <div class="col card" id="baseinfo">
            <div class="col" style="align-items: center;">
                <h2 class="title">问卷配置</h2>
                <div>({{ model.name }})</div>
            </div>
            <div class="col">
                <label class="text-sub fs-xs">问卷标题:</label>
                <input v-model="model.title" placeholder="请输入问卷标题..." />
            </div>
            <div class="row">
                <span class="text-sub">启用状态</span>
                <input type="checkbox" v-model="model.enable" />
            </div>
            <div class="row">
                <span class="text-main">实名提交</span>
                <input type="checkbox" v-model="model.know_who" />
            </div>


            <div @click="addQuestion" id="add" class="button">
                + 添加新题目
            </div>
            <div id="submit" class="button" @click="save">
                保存问卷
            </div>
            <div id="delete" class="button" @click="remoeve">
                删除问卷
            </div>
        </div>
        <div class="scroller" id="questions">
            <div v-for="(q, index) in model.questions" :key="index" class="card">
                <!-- <div class="title">问题{{ index + 1 }}</div> -->
                <input v-model="q.question" placeholder="输入题目内容..." />
                <div class="row">
                    <div class="selector">
                        <div @click="changeKind('Onlyone', index)" :class="{ select: 'Onlyone' in q.choice }">单选</div>
                        <div @click="changeKind('Multiple', index)" :class="{ select: 'Multiple' in q.choice }">多选</div>
                        <div @click="changeKind('Ask', index)" :class="{ select: 'Ask' in q.choice }">咨询</div>
                    </div>
                    <input type="checkbox" v-model="q.required">必填
                </div>
                <template v-if="'Onlyone' in q.choice">
                    <OnlyoneEditor v-model="q.choice.Onlyone.options" />
                </template>

                <template v-else-if="'Multiple' in q.choice">
                    <MultipleEditor v-model="q.choice.Multiple.options" />
                </template>

                <template v-else-if="'Ask' in q.choice">
                    <AskEditor v-model="q.choice.Ask.notification" />
                </template>
                <div @click="removeQuestion(index)" class="button">
                    删除本题
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Table, Question, Choice } from '@/utils/interfaces';
import MultipleEditor from './MultipleEditor.vue';
import AskEditor from './AskEditor.vue';
import OnlyoneEditor from './OnlyoneEditor.vue';

// 接收父组件提供的 table
// 使用 computed 转发 v-model，实现双向绑定
const model = defineModel<Table>();
const emit = defineEmits(['save', 'remove'])
// 添加题目逻辑
const addQuestion = () => {
    if (model.value) {
        const newQuestion: Question = {
            required: false,
            question: '',
            choice: { Onlyone: { options: ["新选项"] } } // 默认添加一个单选题
        };
        model.value.questions.push(newQuestion);
    }
};

// 删除题目逻辑
const removeQuestion = (index: number) => {
    if (!confirm("确认删除这个问题吗")) {
        return
    }
    if (model.value) {
        model.value.questions.splice(index, 1);
    }
};

const changeKind = (kind: "Onlyone" | "Multiple" | "Ask", index: number) => {
    if (model.value) {

        let tem: Choice = { Onlyone: { options: [] } };
        if (kind === "Ask") {
            tem = { Ask: { notification: "" } }
        }
        if (kind === "Multiple") {
            tem = { Multiple: { options: [] } }
        }
        if (model.value.questions[index]) {
            model.value.questions[index].choice = tem
        }
    }
}

const save = () => {
    emit('save')
}

const remoeve = () => {
    emit('remove')
}
</script>

<style scoped>
#table-editor {
    width: 100vw;
    height: 100vh;
    display: flex;
    padding: 20px;
    overflow: hidden;
    gap: 20px;
    /* 防止整体出现滚动条 */
}

#baseinfo {
    width: 500px;
    padding: 20px;
    height: 100%;

}

#list-title {
    grid-area: ls;
    /* 对应 grid-template-areas 中的 ls */
    padding: 0;
    justify-content: flex-start;
    /* 标题靠左 */
}

#questions {
    grid-area: ts;
    /* 对应 grid-template-areas 中的 ts */
    overflow-y: auto;
    /* 只有题目列表部分滚动 */
    padding-right: 10px;
    /* 给滚动条留点空间 */
}

#delete {
    color: rgb(255, 101, 101);
}
</style>