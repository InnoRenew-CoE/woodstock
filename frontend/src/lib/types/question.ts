export type Question = {
  level: number;
  title: string;
  file_id?: number;
  text?: string;
  progress?: string;
  possibleAnswers?: string[];
  answers?: string[];
  type?: QuestionType;
};

export enum QuestionType {
  Text,
  Select,
}
