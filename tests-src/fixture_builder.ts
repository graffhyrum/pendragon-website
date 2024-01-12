import {test as base, expect} from '@playwright/test';

export const test = base.extend({
  page: async ({baseURL, page}, use) => {
    const headshot = page.getByTestId('Headshot');
    await page.goto(baseURL);
    await expect(page).toHaveTitle(/Pendragon Portfolio/);
    await expect(headshot).toBeVisible();
    await use(page);
  }
})