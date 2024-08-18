import {
  FolderIcon,
  FileIcon,
  FileTextIcon,
  FileImageIcon,
  FileVideoIcon,
  FileMusicIcon,
  FileArchiveIcon,
  FileSpreadsheetIcon,
  FileCodeIcon,
  Mail,
  Calendar,
  Database,
  GitBranch,
  FileJson,
  Package,
} from "lucide-svelte";

interface FileInfo {
  name: string;
  is_dir: boolean;
  extension: string | null;
}

export function getIconComponent(file: FileInfo) {
  if (file.is_dir) return FolderIcon;

  // Check for specific filenames first
  const lowercaseName = file.name.toLowerCase();
  if (lowercaseName === "readme.md") return FileTextIcon;
  if (lowercaseName === "license") return FileTextIcon;
  if (lowercaseName === "dockerfile") return FileCodeIcon;
  if (lowercaseName === "package.json") return Package;
  if (lowercaseName === ".gitignore") return GitBranch;

  // Then check extensions
  const extension = file.extension?.toLowerCase();
  switch (extension) {
    // Text files
    case "txt":
    case "md":
    case "rtf":
      return FileTextIcon;

    // Image files
    case "jpg":
    case "jpeg":
    case "png":
    case "gif":
    case "bmp":
    case "svg":
    case "webp":
      return FileImageIcon;

    // Video files
    case "mp4":
    case "avi":
    case "mov":
    case "wmv":
    case "flv":
    case "webm":
      return FileVideoIcon;

    // Audio files
    case "mp3":
    case "wav":
    case "ogg":
    case "flac":
    case "m4a":
      return FileMusicIcon;

    // Archive files
    case "zip":
    case "rar":
    case "7z":
    case "tar":
    case "gz":
      return FileArchiveIcon;

    // Spreadsheet files
    case "xlsx":
    case "xls":
    case "csv":
      return FileSpreadsheetIcon;

    // Code files
    case "js":
    case "ts":
    case "py":
    case "java":
    case "c":
    case "cpp":
    case "html":
    case "css":
    case "php":
      return FileCodeIcon;

    // Database files
    case "sql":
    case "db":
    case "sqlite":
      return Database;

    // Email files
    case "eml":
    case "msg":
      return Mail;

    // Calendar files
    case "ics":
      return Calendar;

    // JSON files
    case "json":
      return FileJson;

    // Default
    default:
      return FileIcon;
  }
}
