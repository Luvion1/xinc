# 26. Compliance

## Standards Compliance

| Standard | Compliance | Verification |
|----------|------------|--------------|
| This document (SPEC.md) | Mandatory | Code review |
| Automated checks | Mandatory | CI pipeline |

## External Standards

| Standard | Applicability | Verification |
|----------|---------------|--------------|
| C11 (ISO/IEC 9899:2011) | Compiler, standard library | Test suite |
| LLVM IR compatibility | All LLVM targets | LLVM verification passes |
| WASM spec | WASM target | Test suite |
| UTF-8 | All text | Encoding check |
| POSIX.1-2017 | POSIX targets | Test suite |
| IEEE 754 | Floating-point operations | Test suite |

## Audit Requirements

| Audit Type | Frequency | Auditor |
|------------|-----------|---------|
| Security | Quarterly | External |
| Code quality | Monthly | Internal |
| Process compliance | Monthly | QA team |

## Definitions

| Term | Definition |
|------|------------|
| **MUST** | Mandatory requirement |
| **SHOULD** | Strongly recommended |
| **MAY** | Optional |
| **Critical Defect** | Security vulnerability, data loss, system crash |
| **Major Defect** | Feature not working, work-around required |
| **Minor Defect** | Cosmetic issue, no work-around needed |
