export function formatBytes(bytes: number) {
	if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
	const units = ['B', 'KB', 'MB', 'GB', 'TB'];
	const index = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
	const value = bytes / 1024 ** index;
	return `${value.toFixed(value >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

export function formatDuration(seconds: number) {
	if (!Number.isFinite(seconds) || seconds <= 0) return '00:00';
	const total = Math.round(seconds);
	const hours = Math.floor(total / 3600);
	const minutes = Math.floor((total % 3600) / 60);
	const secs = total % 60;
	const clock = `${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
	return hours > 0 ? `${hours}:${clock}` : clock;
}

export function fileExtension(path: string) {
	const name = path.split(/[/\\]/).pop() ?? '';
	const index = name.lastIndexOf('.');
	return index >= 0 ? name.slice(index + 1).toLowerCase() : '';
}

export function invokeErrorMessage(error: unknown) {
	if (typeof error === 'string') return error;
	if (error instanceof Error) return error.message;
	return '发生未知错误';
}
