export type ResultChunk = {
  id: string;
  doc_id: string;
  doc_seq_num: number;
  content: string;
  additional_data: string;
  score: number;
};

export enum QuestionType {
  Text,
  Select,
  MultiSelect,
  Tags,
  MultiText,
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
  tags: string[];
};

export type FileAnswer = {
  file: string;
  answers: Answer[];
};
