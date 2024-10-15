import puppeteer from 'puppeteer';
import fs from 'fs';

export async function savePageContent(url: string, filePath: string) {
    // Puppeteerでブラウザ起動
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    
    // 指定されたURLにアクセス
    await page.goto(url, { waitUntil: 'networkidle0' });
    
    // ページ内容を取得
    const content = await page.content();
    
    // ページ内容をHTMLファイルとして保存
    fs.writeFileSync(filePath, content, 'utf8');
    
    // ブラウザを閉じる
    await browser.close();
    
    return { message: 'ページが保存されました。', filePath };
}
