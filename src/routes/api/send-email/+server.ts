// src/routes/api/send-email/+server.ts
import { json } from "@sveltejs/kit";
import nodemailer from "nodemailer";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request }) => {
  try {
    const formData = await request.formData();
    const projectTitle = formData.get("projectTitle") as string;
    const name = formData.get("name") as string;
    const email = formData.get("email") as string;
    const message = formData.get("message") as string;
    const files = formData.getAll("files") as File[];

    // Limit to 3 attachments
    const limitedFiles = files.slice(0, 3);

    let transporter = nodemailer.createTransport({
      service: "Gmail",
      host: "smtp.gmail.com",
      port: 465,
      secure: true,
      auth: {
        user: "anthonyc.animba@gmail.com",
        pass: "dmmh bzfy bjre lfjb",
      },
    });

    // Prepare email attachments
    let attachments = await Promise.all(
      limitedFiles.map(async (file) => {
        const arrayBuffer = await file.arrayBuffer();
        const buffer = Buffer.from(arrayBuffer);
        return {
          filename: file.name,
          content: buffer,
        };
      })
    );

    // Send email
    await transporter.sendMail({
      from: `"${name}" <${email}>`,
      to: "anthonyc.animba@gmail.com",
      subject: `New Project: ${projectTitle}`,
      text: message,
      html: `
                <h1>New Project: ${projectTitle}</h1>
                <p><strong>From:</strong> ${name} (${email})</p>
                <p><strong>Message:</strong></p>
                <p>${message}</p>
            `,
      attachments: attachments,
    });

    return json({ success: true, message: "Email sent successfully" });
  } catch (error) {
    console.error("Error sending email:", error);
    return json(
      { success: false, message: "Failed to send email", error: error.message },
      { status: 500 }
    );
  }
};
