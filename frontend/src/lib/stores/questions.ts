import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { QuestionType, type Answer, type FileAnswer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";
import { pushNotification } from "./notifications";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<FileList | undefined> = writable();
export const questionsStore: Writable<Question[]> = writable([]);
export const tagsStore: Writable<string[]> = writable([]);

export async function fetchQuestions() {
  const request = await fetch(`${PUBLIC_API_BASE_URL}/api/questions`);
  const json = await request.json();
  const { questions, available_tags }: { questions: Question[]; available_tags: string[] } = JSON.parse(json);
  questionsStore.set(questions.toSorted((a, b) => a.id - b.id));
  tagsStore.set(available_tags);
}

export async function submitAnswers(files: FileList, answers: FileAnswer[]) {
  for (let file of Array.from(files)) {
    const correctFileAnswer = answers.find((f) => f.file === file.name);
    if (!correctFileAnswer) {
      console.error(`Unable to find answers for file ${file.name}`);
      continue;
    }
    const formData = new FormData();
    formData.append("answers", JSON.stringify(correctFileAnswer.answers));
    formData.append("file", file, file.name);

    const response = await fetch(`${PUBLIC_API_BASE_URL}/api/answers`, {
      method: "POST",
      body: formData,
    });
  }

  pushNotification({ title: "Success", body: "Submission successful." });
}
