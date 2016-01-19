from ctypes import cdll

import simplejson as json

lib = cdll.LoadLibrary('./libhandler.so')


def handler(event, context):
    ret = lib.handle(json.dumps(event),
                     json.dumps(context, default=encode_context))
    return ret


def encode_context(context):
    return {
        "function_name": context.function_name,
        "function_version": context.function_version,
        "invoked_function_arn": context.invoked_function_arn,
        "memory_limit_in_mb": context.memory_limit_in_mb,
        "aws_request_id": context.aws_request_id,
        "log_group_name": context.log_group_name,
        "log_stream_name": context.log_stream_name

        # add others if it is required.
    }
