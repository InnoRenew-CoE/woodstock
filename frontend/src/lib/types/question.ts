export type Question = {
  id: number;
  file_id?: string;
  title: string;
  text: string;
  possible_answers: Answer[];
  answers?: number[];
  text_answer?: string;
  type: QuestionType;
};

export enum QuestionType {
  Text = "Text",
  Select = "Select",
  MultiSelect = "MultiSelect",
}

export type Answer = {
  id: number;
  value: any;
};
