import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { QuestionType, type Answer, type FileAnswer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";
import { pushNotification } from "./notifications";
import { uploadProgressStore } from "./uploads";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<FileList | undefined> = writable();
export const questionsStore: Writable<Question[]> = writable([]);
export const tagsStore: Writable<string[]> = writable([]);

export async function fetchQuestions() {
  const request = await fetch(`${PUBLIC_API_BASE_URL}/api/questions`);
  const { questions, available_tags }: { questions: Question[]; available_tags: string[] } = await request.json();
  questionsStore.set(questions.toSorted((a, b) => a.id - b.id));
  tagsStore.set(available_tags);
}

export async function submitAnswers(files: FileList, answers: FileAnswer[]) {
  uploadProgressStore.set(
    Array.from(files).map((file) => ({
      file: file.name,
      progress: 0,
    })),
  );
  for (let file of Array.from(files)) {
    const correctFileAnswer = answers.find((f) => f.file === file.name);
    if (!correctFileAnswer) {
      console.error(`Unable to find answers for file ${file.name}`);
      continue;
    }
    const formData = new FormData();
    formData.append("answers", JSON.stringify(correctFileAnswer.answers));
    formData.append("file", file, file.name);

    const xhr = new XMLHttpRequest();
    xhr.open("POST", `${PUBLIC_API_BASE_URL}/api/answers`);
    xhr.upload.onprogress = (event) => {
      if (event.lengthComputable) {
        const percent = (event.loaded / event.total) * 100;
        const percentage = percent.toFixed(2);
        console.log(`Uploaded: ${percentage}%`);
        uploadProgressStore.update((array) => {
          const foundFile = array.find((f) => f.file === file.name);
          if (foundFile) {
            foundFile.progress = percent;
          }
          return array;
        });
      }
    };

    xhr.send(formData);
  }
}
