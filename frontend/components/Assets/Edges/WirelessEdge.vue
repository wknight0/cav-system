<script>
    export const createWirelessEdge = (params, edgeId) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            connection_type: 'wireless',
            animated: false,
            style: {
                stroke: params.color || 'var(--primary-text-color)',
                strokeDasharray: '8 8',
                strokeWidth: 8,
                opacity: 0.8,
            },
        };
    };
</script>
