export type Question = {
  id: number;
  title: string;
  text: string;
  possible_answers: Answer[];
  type: QuestionType;
};

export enum QuestionType {
  Text,
  Select,
  MultiSelect,
}

export type Answer = {
  id: number;
  value: any;
};
