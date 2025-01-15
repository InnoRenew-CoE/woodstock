export enum QuestionType {
  Text,
  Select,
  MultiSelect,
}

export type Question = {
  id: number;
  title: string;
  text: string;
  question_type: QuestionType;
  possible_answers: SelectionAnswer[];
};

export type SelectionAnswer = {
  id: number;
  question_id: number;
  value: string;
};

export type Answer = {
  id?: number;
  question_id: number;
  text?: string;
  selection: number[];
};

export type FileAnswer = {
  file: string;
  answers: Answer[];
};
