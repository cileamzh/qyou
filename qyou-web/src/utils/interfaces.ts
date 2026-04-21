export interface Table {
    name: string
    enable: boolean,
    title: string,
    know_who: boolean
    questions: Question[]
}

export interface Question {
    question: string;
    choice: Choice;
    required: boolean
}

export type Choice =
    | { Onlyone: { options: string[] } }
    | { Multiple: { options: string[] } }
    | { Ask: { notification: string } };

export interface BaseInfo {
    web_title: string
    company: string
}

export interface Who {
    name: string
    identity: string

    ext: string[]
}

export interface AnswerTable {
    name: string
    answers: Answer[]
    who: null | Who
}

export interface Answer {
    question: string
    choice: ChoiceAnswer
}

export type ChoiceAnswer =
    | { Onlyone: { options: string[], answer: number } }
    | { Multiple: { options: string[], answer: number[] } }
    | { Ask: { answer: string } };


export interface TableRecord {
    who: Who | null
    id: string
    date: string,
    answers: Answer[]
}

export const toAnswerTable = (table: Table): AnswerTable => {
    let who: Who | null = null;
    if (table.know_who) {
        who = { name: "", identity: "", ext: [] }
    }
    return {
        name: table.name,
        // 如果允许匿名 (know_who 为 false)，则 who 为 null
        who,
        answers: table.questions.map((q): Answer => {
            const kind = Object.keys(q.choice)[0] as keyof Choice;
            let choiceAnswer: ChoiceAnswer;

            // 根据不同的题目类型初始化答案结构
            if ("Onlyone" in q.choice) {
                choiceAnswer = {
                    Onlyone: {
                        options: q.choice.Onlyone.options,
                        answer: 255 // 默认未选中
                    }
                };
            } else if ("Multiple" in q.choice) {
                choiceAnswer = {
                    Multiple: {
                        options: q.choice.Multiple.options,
                        answer: []    // 默认空数组
                    }
                };
            } else {
                // 默认为 Ask 类型
                choiceAnswer = {
                    Ask: {
                        answer: ""    // 默认空字符串
                    }
                };
            }

            return {
                question: q.question,
                choice: choiceAnswer
            };
        })
    };
};