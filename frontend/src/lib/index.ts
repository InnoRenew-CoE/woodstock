import { PUBLIC_API_BASE_URL } from "$env/static/public";

// place files you want to import through the `$lib` alias in this folder.
export async function verify() {
  const response = await fetch(`${PUBLIC_API_BASE_URL}/api/verify`, {
    method: "post",
    credentials: "include",
  });
  console.log(response);
  return response.status;
}
