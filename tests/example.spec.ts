import {expect, test} from '@playwright/test';
import {goToPage, pages} from "./pages";

test('has title', async ({page}) => {
    await goToPage(page, 'Home');
    await expect(page).toHaveTitle(/Pendragon Portfolio/);
});

test('check routes', async ({page}) => {
    await goToPage(page, 'Home');
    for (const route of pages) {
        await goToPage(page, route.name);
        await expect(async () => {
            expect(page.url(), `Page ${route.name} did not load correctly`)
                .toMatch(route.url)
        }).toPass()
        await expect(page.getByRole('heading', {name: route.name}))
            .toBeVisible()
    }
});

test('check links', async ({page}) => {
    await goToPage(page, 'Home');
    for (const route of pages) {
        const navLink = page
            .getByTestId('NavTree')
            .locator('a', {hasText: route.name})
        await expect(navLink, `Link with name ${route.name} not visible.`).toBeVisible()
        await navLink.click()
        await expect(page.url(),
            `Url for ${route.name} should match ${route.url}, but didn't.`
        ).toMatch(route.url)
    }
})