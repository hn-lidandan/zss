INSERT INTO main_task_messageinfo ("name",description,status,log,priority,task_type,create_timestamp,update_timestamp,bak1,bak2,worker_name) VALUES
	 ('数据备份任务','每周全量备份','RUNNING',NULL,2,'BACKUP','2025-05-19 13:53:01.738','2025-05-19 13:53:01.738',1001,'备用字段',NULL),
	 ('ldd','每周全量备份','SUCCESS',NULL,2,'BACKUP','2025-05-19 14:01:36.222','2025-05-19 14:01:36.222',1001,'备用字段',NULL),
	 ('ldd','测试数据','SUCCESS',NULL,2,'BACKUP','2025-05-19 16:27:35.970','2025-05-19 16:27:35.970',1001,'备用字段',NULL),
	 ('乱七八糟','测试数据','SUCCESS',NULL,2,'BACKUP','2025-05-19 16:28:19.041','2025-05-19 16:28:19.041',1001,'备用字段',NULL),
	 ('','11软件更新6666661111','created',NULL,1,'add','2025-05-29 11:15:47.076','2025-05-29 11:15:47.076',NULL,NULL,NULL);



CREATE TABLE main_task_messageinfo (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(255) NOT NULL,
    log TEXT,
    priority INTEGER ,
    task_type VARCHAR(255),
    worker_name VARCHAR NOT NULL,
    create_timestamp TIMESTAMP ,
    update_timestamp TIMESTAMP,
    bak1 BIGINT,
    bak2 TEXT
);


-- 创建任务表
CREATE TABLE main_task_messageinfo (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    status   VARCHAR(50) NOT NULL,
    log TEXT,
    priority INTEGER ,
    task_type VARCHAR(50) NOT NULL,
    worker_name VARCHAR(255) NOT NULL,
    create_timestamp TIMESTAMPTZ  NOT NULL,
    update_timestamp TIMESTAMPTZ ,
    bak1 BIGINT,
    bak2 VARCHAR(255)
    
);


-- 添加字段注释
COMMENT ON COLUMN main_task_messageinfo.id IS '任务唯一标识';
COMMENT ON COLUMN main_task_messageinfo.name IS '任务名称';
COMMENT ON COLUMN main_task_messageinfo.description IS '任务详细描述';
COMMENT ON COLUMN main_task_messageinfo.status IS '任务状态';
COMMENT ON COLUMN main_task_messageinfo.log IS '任务执行信息';
COMMENT ON COLUMN main_task_messageinfo.priority IS '任务优先级';
COMMENT ON COLUMN main_task_messageinfo.task_type IS '任务类型标识';
COMMENT ON COLUMN main_task_messageinfo.worker_name IS 'worker名称';
COMMENT ON COLUMN main_task_messageinfo.create_timestamp IS '任务创建时间';
COMMENT ON COLUMN main_task_messageinfo.update_timestamp IS '最后更新时间';
COMMENT ON COLUMN main_task_messageinfo.bak1 IS '备用字段1';
COMMENT ON COLUMN main_task_messageinfo.bak2 IS '备用字段2';



ALTER TABLE main_task_messageinfo 
ADD COLUMN worker_name VARCHAR(255) ;

