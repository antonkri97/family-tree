-- Add migration script here
-- Шаг 1: Добавляем временную колонку типа BOOLEAN
ALTER TABLE users ADD COLUMN verified_temp BOOLEAN;

-- Шаг 2: Обновляем значения в новой колонке на основе старой колонки
UPDATE users SET verified_temp = (verified = 'true');

-- Шаг 3: Удаляем старую колонку verified
ALTER TABLE users DROP COLUMN verified;

-- Шаг 4: Переименовываем временную колонку в verified
ALTER TABLE users RENAME COLUMN verified_temp TO verified;

-- Шаг 5: Устанавливаем NOT NULL для новой колонки verified
ALTER TABLE users ALTER COLUMN verified SET NOT NULL;