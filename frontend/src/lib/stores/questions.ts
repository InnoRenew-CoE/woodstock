import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { QuestionType, type Answer, type FileAnswer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";
import { pushNotification } from "./notifications";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<FileList | undefined> = writable();
export const questionsStore: Writable<Question[]> = writable([]);

export async function fetchQuestions() {
  const request = await fetch(`${PUBLIC_API_BASE_URL}/api/questions`);
  const json = await request.json();
  const questions: Question[] = JSON.parse(json);
  questionsStore.set(questions);
}

export async function submitAnswers(files: FileList) {
  for (let file of Array.from(files)) {
    const formData = new FormData();
    formData.append(
      "answers",
      JSON.stringify([
        { question_id: 1, text: "AE", selection: [] },
        { question_id: 2, selection: [1] },
      ]),
    );
    formData.append("file", file, file.name);

    const response = await fetch(`${PUBLIC_API_BASE_URL}/api/answers`, {
      method: "POST",
      body: formData,
    });
  }

  pushNotification({ title: "Success", body: "Submission successful." });
}
