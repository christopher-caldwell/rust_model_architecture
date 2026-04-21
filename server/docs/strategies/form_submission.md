# Form Submission Strategy

## Overview

Forms are submitted via a client-side `fetch` call with a URL-encoded body (`URLSearchParams`). On success, the user is redirected to `/thank-you`. On failure, a generic error message is shown in-place — no redirect.

---

## File Structure

### `_features/<feature>/api/submit.ts`

Contains all HTTP request logic. Nothing outside this file should call `fetch` for form submission.

**Responsibilities:**
- Accept `(url: string, form: HTMLFormElement)` and return `Promise<boolean>`
- Build a `FormData` from the form element
- Transform fields as needed (e.g. split `name` → `first_name` + `last_name`)
- Serialize to `URLSearchParams`
- POST to the provided URL
- Return `res.ok`

**Helpers defined here (private, not exported):**
- `splitName(raw: string)` — splits a full name string into `{ firstName, lastName }`, accounting for suffixes (Jr., Sr., PhD, etc.) and surname prefixes (de, van, von, etc.)
- `extractVal(formData, fieldName)` — shorthand for `String(formData.get(fieldName) ?? '').trim()`

**Example:**
```ts
export async function submitForm(url: string, form: HTMLFormElement): Promise<boolean> {
  const formData = new FormData(form);
  const { firstName, lastName } = splitName(String(formData.get('name') ?? ''));

  const params = new URLSearchParams();
  params.append(ContactFormFields.FirstName, firstName);
  params.append(ContactFormFields.LastName, lastName);
  params.append(ContactFormFields.Phone, extractVal(formData, ContactFormFields.Phone));
  // ... remaining fields
  params.append(ContactFormFields.Source, 'connect');

  const res = await fetch(url, { method: 'POST', body: params });
  return res.ok;
}
```

---

### `_features/<feature>/util/` (e.g. `sheet.ts`)

Contains UI/interaction logic. Imports `submitForm` from `../api/submit`.

**Responsibilities:**
- Listen for the form `submit` event and call `e.preventDefault()`
- Manage loading state (disable button, show spinner)
- Call `submitForm(url, form)`
- On success: redirect to `/thank-you`
- On failure: show a generic error message in-place, do not redirect
- Restore button/loading state in `finally`

**The submission URL** comes from `root.dataset.formUrl` (set as a `data-form-url` attribute on the feature root element in the Astro template), not from `form.action`.

**Example pattern:**
```ts
const ok = await submitForm(formSubmissionUrl, el.form);
if (!ok) {
  el.formError.textContent = 'Something went wrong. Please try again later.';
  el.formError.classList.remove('hidden');
  return;
}
window.location.href = '/thank-you';
```

---

## Payload Shape

All field names use the `ContactFormFields` enum from `@/constants`. The `name` input on the form is a single combined field — it is split into `first_name` and `last_name` before submission. The payload always includes `source: 'connect'`.

| Enum Key    | Wire Name       | Source               |
|-------------|-----------------|----------------------|
| `FirstName` | `first_name`    | derived from `name`  |
| `LastName`  | `last_name`     | derived from `name`  |
| `Phone`     | `phone_number`  | form field           |
| `Email`     | `email`         | form field           |
| `Website`   | `website_given` | form field           |
| `Message`   | `message`       | form field           |
| `Url`       | `url`           | honeypot field       |
| `Source`    | `source`        | hardcoded `connect`  |

---

## Thank You Page

`/thank-you` is a simple standalone page with:
- Heading: "Nice to meet you!"
- Subtitle: "I'm looking forward to connecting with you."
- A "Back" button linking to `/`
