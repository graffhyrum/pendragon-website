import {Page} from "@playwright/test";

export const pages = [
    {name: 'Home', url: '/'},
    {name: 'My Work', url: '/my_work.html'},
    {name: 'Bookshelf', url: '/bookshelf.html'},
    {name: "Testimonials", url: "/testimonials.html"},
] satisfies Array<{ name: string, url: string }>;

export async function goToPage(page: Page, name: typeof pages[number]['name']) {
    const targetPage = pages.find(
        p => p.name === name
    );
    if (!targetPage) throw new Error(`No page found with name ${name}`);
    await page.goto(targetPage.url);
}