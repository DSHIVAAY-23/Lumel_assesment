-- **Section C: Data Persistence**
-- **Database – SQL – Medium**
-- **Driver Delivery Performance Analysis**

-- Problem Statement:
-- You have a database table deliveries tracking package drop-offs.
-- Schema:
-- • `id` (int, PK)
-- • `driver_id` (int)
-- • `delivery_status` (varchar: 'COMPLETED', 'FAILED', 'LATE')
-- • `attempt_timestamp` (datetime)
-- Write a SQL query to identify "Reliable Drivers".
-- A "Reliable Driver" is defined as a driver who has:
-- 1. Completed at least 5 deliveries in the month of **September 2024**.
-- 2. Maintained a 'COMPLETED' rate of **90% or higher** for that month.
-- Output Columns: driver_id, total_deliveries, success_rate (percentage).
-- Ordering: Order by success_rate descending.



SELECT
      driver_id,
      COUNT(*) As total_deliveries,
      ROUND(SUM(CASE WHEN delivery_status = 'COMPLETED' THEN 1 ELSE 0 END)*100.0)/COUNT(*),2) AS success_rate
FROM deliveries      
WHERE
     attempt_timestamp >='2024-09-01'
     AND attempt_timestamp < '2024-10-01'
GROUP BY driver_id
HAVING
     COUNT(*) >= 5
     AND (SUM(CASE WHEN delivery_status ='COMPLETED' THEN 1 ELSE 0 END)*100)/COUNT(*)>=90     
ORDER BY success_rate DESC